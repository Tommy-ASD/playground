DEFINE SCOPE account SESSION 1d SIGNIN {
    LET $userToken = http::post('https://github.com/login/oauth/access_token', {
        "client_id": "GITHUB_CLIENT_ID",
        "client_secret": "GITHUB_SECRET_ID",
        "code": $code
    }, {
        "Accept": "application/vnd.github+json",
    });

    LET $userInfo = http::get('https://api.github.com/user', {
        "Accept": "application/vnd.github+json",
        "Authorization": string::concat("Bearer ", $userToken.access_token)
    });

    LET $userID = type::thing("users", ['github', $userInfo.type, $userInfo.id ]);

    LET $user = SELECT * FROM $userID;

    IF $user[0].id == NONE THEN 
        CREATE $userID CONTENT {
            name: $userInfo.name,
            avatar_url: $userInfo.avatar_url,
            profil_url: $userInfo.html_url
        }
    ELSE IF 
        [$user[0].name, $user[0].avatar_url, $user[0].profil_url] != 
        [$userInfo.name, $userInfo.avatar_url, $userInfo.html_url] THEN
        UPDATE $userID SET 
            name = $userInfo.name, 
            avatar_url = $userInfo.avatar_url, 
            profil_url = $userInfo.html_url;
    ELSE
        RETURN $user[0];
    END;

    RETURN (SELECT * FROM $userID)[0];
}