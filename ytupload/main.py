import os
import re
from googleapiclient.discovery import build
from googleapiclient.http import MediaFileUpload
from google_auth_oauthlib.flow import InstalledAppFlow

# directory to search for video files
VIDEO_DIR = '/Medal/Clips'

SCOPES = ["https://www.googleapis.com/auth/youtube.upload"]


API_KEY = 'AIzaSyC9MvajQM2bvUUg-3OdWsGtvaE2qRzqMv4'

def authenticate_youtube():
    """Authenticate and build the YouTube API client."""
    flow = InstalledAppFlow.from_client_secrets_file('client_secret.json', SCOPES)
    credentials = flow.run_local_server()
    youtube = build('youtube', 'v3', credentials=credentials)
    return youtube

def upload_video(youtube, file_path, title, description, tags, privacy_status):
    """Upload a video to YouTube."""
    body = {
        'snippet': {
            'title': title,
            'description': description,
            'tags': tags,
            'categoryId': '22',  # Category ID for "People & Blogs"
        },
        'status': {
            'privacyStatus': privacy_status,  # Options: public, private, unlisted
        }
    }
    
    # Media upload
    media = MediaFileUpload(file_path, chunksize=-1, resumable=True)
    request = youtube.videos().insert(
        part="snippet,status",
        body=body,
        media_body=media
    )

    # Upload video and handle response
    response = request.execute()
    print(f"Video uploaded: {response['id']}")
    return response

def search_files(directory, extensions=('.mp4', '.avi', '.mov')):
    """Recursively search for video files in a directory."""
    video_files = []
    for root, dirs, files in os.walk(directory):
        for file in files:
            if file.lower().endswith(extensions):
                video_files.append(os.path.join(root, file))
    return video_files

def main():
    # Authenticate YouTube API
    youtube = authenticate_youtube()

    # Search for video files
    video_files = search_files(VIDEO_DIR)

    # Loop through each video file and upload
    for video_path in video_files:
        try:
            # Extract video title from the filename (remove file extension)
            video_title = os.path.splitext(os.path.basename(video_path))[0]
            video_description = f"Vod uploaded from {video_path}"
            video_tags = []  # You can customize tags
            privacy_status = 'private'  # Change to 'private' or 'unlisted' as needed
            
            # Upload the video
            upload_video(youtube, video_path, video_title, video_description, video_tags, privacy_status)

        except Exception as e:
            print(f"Error uploading {video_path}: {e}")

if __name__ == '__main__':
    main()
