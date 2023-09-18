use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum ProgrammingLanguage {
    Abap,         // ("abap")
    Ada,          // ("adb", "ads", "ada")
    Ahk,          // ("ahk", "ahkl")
    Apacheconf,   // ("htaccess", "apacheconf", "apache2conf")
    Applescript,  // ("applescript")
    As,           // ("as")
    As3,          // ("as")
    Asy,          // ("asy")
    Bash,         // ("sh", "ksh", "bash", "ebuild", "eclass")
    Bat,          // ("bat", "cmd")
    Befunge,      // ("befunge")
    Blitzmax,     // ("bmx")
    Boo,          // ("boo")
    Brainfuck,    // ("bf", "b")
    C,            // ("c", "h")
    Cfm,          // ("cfm", "cfml", "cfc")
    Cheetah,      // ("tmpl", "spt")
    Cl,           // ("cl", "lisp", "el")
    Clojure,      // ("clj", "cljs")
    Cmake,        // ("cmake", "CMakeListstxt")
    Coffeescript, // ("coffee")
    Console,      // ("sh-session")
    Control,      // ("control")
    Cpp,          // ("cpp", "hpp", "c++", "h++", "cc", "hh", "cxx", "hxx", "pde")
    Csharp,       // ("cs")
    Css,          // ("css")
    Cucumber,     // ("feature")
    Cython,       // ("pyx", "pxd", "pxi")
    D,            // ("d", "di")
    Delphi,       // ("pas")
    Diff,         // ("diff", "patch")
    Dpatch,       // ("dpatch", "darcspatch")
    Duel,         // ("duel", "jbst")
    Dylan,        // ("dylan", "dyl")
    Erb,          // ("erb")
    Erl,          // ("erl-sh")
    Erlang,       // ("erl", "hrl")
    Evoque,       // ("evoque")
    Factor,       // ("factor")
    Felix,        // ("flx", "flxh")
    Fortran,      // ("f", "f90")
    Gas,          // ("s", "S")
    Genshi,       // ("kid")
    Glsl,         // ("vert", "frag", "geo")
    Gnuplot,      // ("plot", "plt")
    Go,           // ("go")
    Groff,        // ("(1234567)", "man")
    Haml,         // ("haml")
    Haskell,      // ("hs")
    Html,         // ("html", "htm", "xhtml", "xslt")
    Hx,           // ("hx")
    Hybris,       // ("hy", "hyb")
    Ini,          // ("ini", "cfg")
    Io,           // ("io")
    Ioke,         // ("ik")
    Irc,          // ("weechatlog")
    Jade,         // ("jade")
    Java,         // ("java")
    Js,           // ("js")
    Jsp,          // ("jsp")
    Lhs,          // ("lhs")
    Llvm,         // ("ll")
    Logtalk,      // ("lgt")
    Lua,          // ("lua", "wlua")
    Make,         // ("mak", "Makefile", "makefile", "Makefile", "GNUmakefile")
    Mako,         // ("mao")
    Maql,         // ("maql")
    Mason,        // ("mhtml", "mc", "mi", "autohandler", "dhandler")
    Markdown,     // ("md")
    Modelica,     // ("mo")
    Modula2,      // ("def", "mod")
    Moocode,      // ("moo")
    Mupad,        // ("mu")
    Mxml,         // ("mxml")
    Myghty,       // ("myt", "autodelegate")
    Nasm,         // ("asm", "ASM")
    Newspeak,     // ("ns2")
    Objdump,      // ("objdump")
    Objectivec,   // ("m")
    Objectivej,   // ("j")
    Ocaml,        // ("ml", "mli", "mll", "mly")
    Ooc,          // ("ooc")
    Perl,         // ("pl", "pm")
    Php,          // ("php", "php(345)")
    Postscript,   // ("ps", "eps")
    Pot,          // ("pot", "po")
    Pov,          // ("pov", "inc")
    Prolog,       // ("prolog", "pro", "pl")
    Properties,   // ("properties")
    Protobuf,     // ("proto")
    Py3tb,        // ("py3tb")
    Pytb,         // ("pytb")
    Python,       // ("py", "pyw", "sc", "SConstruct", "SConscript", "tac")
    R,            // ("R")
    Rb,           // ("rb", "rbw", "Rakefile", "rake", "gemspec", "rbx", "duby")
    Rconsole,     // ("Rout")
    Rebol,        // ("r", "r3")
    Redcode,      // ("cw")
    Rhtml,        // ("rhtml")
    Rst,          // ("rst", "rest")
    Sass,         // ("sass")
    Scala,        // ("scala")
    Scaml,        // ("scaml")
    Scheme,       // ("scm")
    Scss,         // ("scss")
    Smalltalk,    // ("st")
    Smarty,       // ("tpl")
    Sourceslist,  // ("sourceslist")
    Splus,        // ("S", "R")
    Sql,          // ("sql")
    Sqlite3,      // ("sqlite3-console")
    Squidconf,    // ("squidconf")
    Ssp,          // ("ssp")
    Tcl,          // ("tcl")
    Tcsh,         // ("tcsh", "csh")
    Tex,          // ("tex", "aux", "toc")
    Text,         // ("txt")
    V,            // ("v", "sv")
    Vala,         // ("vala", "vapi")
    Vbnet,        // ("vb", "bas")
    Velocity,     // ("vm", "fhtml")
    Vim,          // ("vim", "vimrc")
    Xml,          // ("xml", "xsl", "rss", "xslt", "xsd", "wsdl")
    Xquery,       // ("xqy", "xquery")
    Xslt,         // ("xsl", "xslt")
    Yaml,         // ("yaml", "yml")
    Unknown,      // ("*")
}

impl ProgrammingLanguage {
    pub fn to_extension<'a>(&'a self) -> Vec<&'a str> {
        match self {
            &ProgrammingLanguage::Abap => vec!["abap"],
            &ProgrammingLanguage::Ada => vec!["adb", "ads", "ada"],
            &ProgrammingLanguage::Ahk => vec!["ahk", "ahkl"],
            &ProgrammingLanguage::Apacheconf => vec!["htaccess", "apacheconf", "apache2conf"],
            &ProgrammingLanguage::Applescript => vec!["applescript"],
            &ProgrammingLanguage::As => vec!["as"],
            &ProgrammingLanguage::As3 => vec!["as"],
            &ProgrammingLanguage::Asy => vec!["asy"],
            &ProgrammingLanguage::Bash => vec!["sh", "ksh", "bash", "ebuild", "eclass"],
            &ProgrammingLanguage::Bat => vec!["bat", "cmd"],
            &ProgrammingLanguage::Befunge => vec!["befunge"],
            &ProgrammingLanguage::Blitzmax => vec!["bmx"],
            &ProgrammingLanguage::Boo => vec!["boo"],
            &ProgrammingLanguage::Brainfuck => vec!["bf", "b"],
            &ProgrammingLanguage::C => vec!["c", "h"],
            &ProgrammingLanguage::Cfm => vec!["cfm", "cfml", "cfc"],
            &ProgrammingLanguage::Cheetah => vec!["tmpl", "spt"],
            &ProgrammingLanguage::Cl => vec!["cl", "lisp", "el"],
            &ProgrammingLanguage::Clojure => vec!["clj", "cljs"],
            &ProgrammingLanguage::Cmake => vec!["cmake", "CMakeListstxt"],
            &ProgrammingLanguage::Coffeescript => vec!["coffee"],
            &ProgrammingLanguage::Console => vec!["sh-session"],
            &ProgrammingLanguage::Control => vec!["control"],
            &ProgrammingLanguage::Cpp => {
                vec!["cpp", "hpp", "c++", "h++", "cc", "hh", "cxx", "hxx", "pde"]
            }
            &ProgrammingLanguage::Csharp => vec!["cs"],
            &ProgrammingLanguage::Css => vec!["css"],
            &ProgrammingLanguage::Cucumber => vec!["feature"],
            &ProgrammingLanguage::Cython => vec!["pyx", "pxd", "pxi"],
            &ProgrammingLanguage::D => vec!["d", "di"],
            &ProgrammingLanguage::Delphi => vec!["pas"],
            &ProgrammingLanguage::Diff => vec!["diff", "patch"],
            &ProgrammingLanguage::Dpatch => vec!["dpatch", "darcspatch"],
            &ProgrammingLanguage::Duel => vec!["duel", "jbst"],
            &ProgrammingLanguage::Dylan => vec!["dylan", "dyl"],
            &ProgrammingLanguage::Erb => vec!["erb"],
            &ProgrammingLanguage::Erl => vec!["erl-sh"],
            &ProgrammingLanguage::Erlang => vec!["erl", "hrl"],
            &ProgrammingLanguage::Evoque => vec!["evoque"],
            &ProgrammingLanguage::Factor => vec!["factor"],
            &ProgrammingLanguage::Felix => vec!["flx", "flxh"],
            &ProgrammingLanguage::Fortran => vec!["f", "f90"],
            &ProgrammingLanguage::Gas => vec!["s", "S"],
            &ProgrammingLanguage::Genshi => vec!["kid"],
            &ProgrammingLanguage::Glsl => vec!["vert", "frag", "geo"],
            &ProgrammingLanguage::Gnuplot => vec!["plot", "plt"],
            &ProgrammingLanguage::Go => vec!["go"],
            &ProgrammingLanguage::Groff => vec!["(1234567)", "man"],
            &ProgrammingLanguage::Haml => vec!["haml"],
            &ProgrammingLanguage::Haskell => vec!["hs"],
            &ProgrammingLanguage::Html => vec!["html", "htm", "xhtml", "xslt"],
            &ProgrammingLanguage::Hx => vec!["hx"],
            &ProgrammingLanguage::Hybris => vec!["hy", "hyb"],
            &ProgrammingLanguage::Ini => vec!["ini", "cfg"],
            &ProgrammingLanguage::Io => vec!["io"],
            &ProgrammingLanguage::Ioke => vec!["ik"],
            &ProgrammingLanguage::Irc => vec!["weechatlog"],
            &ProgrammingLanguage::Jade => vec!["jade"],
            &ProgrammingLanguage::Java => vec!["java"],
            &ProgrammingLanguage::Js => vec!["js"],
            &ProgrammingLanguage::Jsp => vec!["jsp"],
            &ProgrammingLanguage::Lhs => vec!["lhs"],
            &ProgrammingLanguage::Llvm => vec!["ll"],
            &ProgrammingLanguage::Logtalk => vec!["lgt"],
            &ProgrammingLanguage::Lua => vec!["lua", "wlua"],
            &ProgrammingLanguage::Make => {
                vec!["mak", "Makefile", "makefile", "Makefile", "GNUmakefile"]
            }
            &ProgrammingLanguage::Mako => vec!["mao"],
            &ProgrammingLanguage::Maql => vec!["maql"],
            &ProgrammingLanguage::Mason => vec!["mhtml", "mc", "mi", "autohandler", "dhandler"],
            &ProgrammingLanguage::Markdown => vec!["md"],
            &ProgrammingLanguage::Modelica => vec!["mo"],
            &ProgrammingLanguage::Modula2 => vec!["def", "mod"],
            &ProgrammingLanguage::Moocode => vec!["moo"],
            &ProgrammingLanguage::Mupad => vec!["mu"],
            &ProgrammingLanguage::Mxml => vec!["mxml"],
            &ProgrammingLanguage::Myghty => vec!["myt", "autodelegate"],
            &ProgrammingLanguage::Nasm => vec!["asm", "ASM"],
            &ProgrammingLanguage::Newspeak => vec!["ns2"],
            &ProgrammingLanguage::Objdump => vec!["objdump"],
            &ProgrammingLanguage::Objectivec => vec!["m"],
            &ProgrammingLanguage::Objectivej => vec!["j"],
            &ProgrammingLanguage::Ocaml => vec!["ml", "mli", "mll", "mly"],
            &ProgrammingLanguage::Ooc => vec!["ooc"],
            &ProgrammingLanguage::Perl => vec!["pl", "pm"],
            &ProgrammingLanguage::Php => vec!["php", "php(345"],
            &ProgrammingLanguage::Postscript => vec!["ps", "eps"],
            &ProgrammingLanguage::Pot => vec!["pot", "po"],
            &ProgrammingLanguage::Pov => vec!["pov", "inc"],
            &ProgrammingLanguage::Prolog => vec!["prolog", "pro", "pl"],
            &ProgrammingLanguage::Properties => vec!["properties"],
            &ProgrammingLanguage::Protobuf => vec!["proto"],
            &ProgrammingLanguage::Py3tb => vec!["py3tb"],
            &ProgrammingLanguage::Pytb => vec!["pytb"],
            &ProgrammingLanguage::Python => {
                vec!["py", "pyw", "sc", "SConstruct", "SConscript", "tac"]
            }
            &ProgrammingLanguage::R => vec!["R"],
            &ProgrammingLanguage::Rb => {
                vec!["rb", "rbw", "Rakefile", "rake", "gemspec", "rbx", "duby"]
            }
            &ProgrammingLanguage::Rconsole => vec!["Rout"],
            &ProgrammingLanguage::Rebol => vec!["r", "r3"],
            &ProgrammingLanguage::Redcode => vec!["cw"],
            &ProgrammingLanguage::Rhtml => vec!["rhtml"],
            &ProgrammingLanguage::Rst => vec!["rst", "rest"],
            &ProgrammingLanguage::Sass => vec!["sass"],
            &ProgrammingLanguage::Scala => vec!["scala"],
            &ProgrammingLanguage::Scaml => vec!["scaml"],
            &ProgrammingLanguage::Scheme => vec!["scm"],
            &ProgrammingLanguage::Scss => vec!["scss"],
            &ProgrammingLanguage::Smalltalk => vec!["st"],
            &ProgrammingLanguage::Smarty => vec!["tpl"],
            &ProgrammingLanguage::Sourceslist => vec!["sourceslist"],
            &ProgrammingLanguage::Splus => vec!["S", "R"],
            &ProgrammingLanguage::Sql => vec!["sql"],
            &ProgrammingLanguage::Sqlite3 => vec!["sqlite3-console"],
            &ProgrammingLanguage::Squidconf => vec!["squidconf"],
            &ProgrammingLanguage::Ssp => vec!["ssp"],
            &ProgrammingLanguage::Tcl => vec!["tcl"],
            &ProgrammingLanguage::Tcsh => vec!["tcsh", "csh"],
            &ProgrammingLanguage::Tex => vec!["tex", "aux", "toc"],
            &ProgrammingLanguage::Text => vec!["txt"],
            &ProgrammingLanguage::V => vec!["v", "sv"],
            &ProgrammingLanguage::Vala => vec!["vala", "vapi"],
            &ProgrammingLanguage::Vbnet => vec!["vb", "bas"],
            &ProgrammingLanguage::Velocity => vec!["vm", "fhtml"],
            &ProgrammingLanguage::Vim => vec!["vim", "vimrc"],
            &ProgrammingLanguage::Xml => vec!["xml", "xsl", "rss", "xslt", "xsd", "wsdl"],
            &ProgrammingLanguage::Xquery => vec!["xqy", "xquery"],
            &ProgrammingLanguage::Xslt => vec!["xsl", "xslt"],
            &ProgrammingLanguage::Yaml => vec!["yaml", "yml"],
            &ProgrammingLanguage::Unknown => vec![],
        }
    }
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "abap" => ProgrammingLanguage::Abap,
            "adb" | "ads" | "ada" => ProgrammingLanguage::Ada,
            "ahk" | "ahkl" => ProgrammingLanguage::Ahk,
            "htaccess" | "apacheconf" | "apache2conf" => ProgrammingLanguage::Apacheconf,
            "applescript" => ProgrammingLanguage::Applescript,
            "as" => ProgrammingLanguage::As,
            "asy" => ProgrammingLanguage::Asy,
            "sh" | "ksh" | "bash" | "ebuild" | "eclass" => ProgrammingLanguage::Bash,
            "bat" | "cmd" => ProgrammingLanguage::Bat,
            "befunge" => ProgrammingLanguage::Befunge,
            "bmx" => ProgrammingLanguage::Blitzmax,
            "boo" => ProgrammingLanguage::Boo,
            "bf" | "b" => ProgrammingLanguage::Brainfuck,
            "c" | "h" => ProgrammingLanguage::C,
            "cfm" | "cfml" | "cfc" => ProgrammingLanguage::Cfm,
            "tmpl" | "spt" => ProgrammingLanguage::Cheetah,
            "cl" | "lisp" | "el" => ProgrammingLanguage::Cl,
            "clj" | "cljs" => ProgrammingLanguage::Clojure,
            "cmake" | "CMakeListstxt" => ProgrammingLanguage::Cmake,
            "coffee" => ProgrammingLanguage::Coffeescript,
            "sh-session" => ProgrammingLanguage::Console,
            "control" => ProgrammingLanguage::Control,
            "cpp" | "hpp" | "c++" | "h++" | "cc" | "hh" | "cxx" | "hxx" | "pde" => {
                ProgrammingLanguage::Cpp
            }
            "cs" => ProgrammingLanguage::Csharp,
            "css" => ProgrammingLanguage::Css,
            "feature" => ProgrammingLanguage::Cucumber,
            "pyx" | "pxd" | "pxi" => ProgrammingLanguage::Cython,
            "d" | "di" => ProgrammingLanguage::D,
            "pas" => ProgrammingLanguage::Delphi,
            "diff" | "patch" => ProgrammingLanguage::Diff,
            "dpatch" | "darcspatch" => ProgrammingLanguage::Dpatch,
            "duel" | "jbst" => ProgrammingLanguage::Duel,
            "dylan" | "dyl" => ProgrammingLanguage::Dylan,
            "erb" => ProgrammingLanguage::Erb,
            "erl-sh" => ProgrammingLanguage::Erl,
            "erl" | "hrl" => ProgrammingLanguage::Erlang,
            "evoque" => ProgrammingLanguage::Evoque,
            "factor" => ProgrammingLanguage::Factor,
            "flx" | "flxh" => ProgrammingLanguage::Felix,
            "f" | "f90" => ProgrammingLanguage::Fortran,
            "s" => ProgrammingLanguage::Gas,
            "kid" => ProgrammingLanguage::Genshi,
            "vert" | "frag" | "geo" => ProgrammingLanguage::Glsl,
            "plot" | "plt" => ProgrammingLanguage::Gnuplot,
            "go" => ProgrammingLanguage::Go,
            "(1234567)" | "man" => ProgrammingLanguage::Groff,
            "haml" => ProgrammingLanguage::Haml,
            "hs" => ProgrammingLanguage::Haskell,
            "html" | "htm" | "xhtml" => ProgrammingLanguage::Html,
            "hx" => ProgrammingLanguage::Hx,
            "hy" | "hyb" => ProgrammingLanguage::Hybris,
            "ini" | "cfg" => ProgrammingLanguage::Ini,
            "io" => ProgrammingLanguage::Io,
            "ik" => ProgrammingLanguage::Ioke,
            "weechatlog" => ProgrammingLanguage::Irc,
            "jade" => ProgrammingLanguage::Jade,
            "java" => ProgrammingLanguage::Java,
            "js" => ProgrammingLanguage::Js,
            "jsp" => ProgrammingLanguage::Jsp,
            "lhs" => ProgrammingLanguage::Lhs,
            "ll" => ProgrammingLanguage::Llvm,
            "lgt" => ProgrammingLanguage::Logtalk,
            "lua" | "wlua" => ProgrammingLanguage::Lua,
            "mak" | "Makefile" | "makefile" | "GNUmakefile" => ProgrammingLanguage::Make,
            "mao" => ProgrammingLanguage::Mako,
            "maql" => ProgrammingLanguage::Maql,
            "mhtml" | "mc" | "mi" | "autohandler" | "dhandler" => ProgrammingLanguage::Mason,
            "md" => ProgrammingLanguage::Markdown,
            "mo" => ProgrammingLanguage::Modelica,
            "def" | "mod" => ProgrammingLanguage::Modula2,
            "moo" => ProgrammingLanguage::Moocode,
            "mu" => ProgrammingLanguage::Mupad,
            "mxml" => ProgrammingLanguage::Mxml,
            "myt" | "autodelegate" => ProgrammingLanguage::Myghty,
            "asm" | "ASM" => ProgrammingLanguage::Nasm,
            "ns2" => ProgrammingLanguage::Newspeak,
            "objdump" => ProgrammingLanguage::Objdump,
            "m" => ProgrammingLanguage::Objectivec,
            "j" => ProgrammingLanguage::Objectivej,
            "ml" | "mli" | "mll" | "mly" => ProgrammingLanguage::Ocaml,
            "ooc" => ProgrammingLanguage::Ooc,
            "pl" | "pm" => ProgrammingLanguage::Perl,
            "php" | "php(345" => ProgrammingLanguage::Php,
            "ps" | "eps" => ProgrammingLanguage::Postscript,
            "pot" | "po" => ProgrammingLanguage::Pot,
            "pov" | "inc" => ProgrammingLanguage::Pov,
            "prolog" | "pro" => ProgrammingLanguage::Prolog,
            "properties" => ProgrammingLanguage::Properties,
            "proto" => ProgrammingLanguage::Protobuf,
            "py3tb" => ProgrammingLanguage::Py3tb,
            "pytb" => ProgrammingLanguage::Pytb,
            "py" | "pyw" | "sc" | "SConstruct" | "SConscript" | "tac" => {
                ProgrammingLanguage::Python
            }
            "R" => ProgrammingLanguage::R,
            "rb" | "rbw" | "Rakefile" | "rake" | "gemspec" | "rbx" | "duby" => {
                ProgrammingLanguage::Rb
            }
            "Rout" => ProgrammingLanguage::Rconsole,
            "r" | "r3" => ProgrammingLanguage::Rebol,
            "cw" => ProgrammingLanguage::Redcode,
            "rhtml" => ProgrammingLanguage::Rhtml,
            "rst" | "rest" => ProgrammingLanguage::Rst,
            "sass" => ProgrammingLanguage::Sass,
            "scala" => ProgrammingLanguage::Scala,
            "scaml" => ProgrammingLanguage::Scaml,
            "scm" => ProgrammingLanguage::Scheme,
            "scss" => ProgrammingLanguage::Scss,
            "st" => ProgrammingLanguage::Smalltalk,
            "tpl" => ProgrammingLanguage::Smarty,
            "sourceslist" => ProgrammingLanguage::Sourceslist,
            "S" => ProgrammingLanguage::Splus,
            "sql" => ProgrammingLanguage::Sql,
            "sqlite3-console" => ProgrammingLanguage::Sqlite3,
            "squidconf" => ProgrammingLanguage::Squidconf,
            "ssp" => ProgrammingLanguage::Ssp,
            "tcl" => ProgrammingLanguage::Tcl,
            "tcsh" | "csh" => ProgrammingLanguage::Tcsh,
            "tex" | "aux" | "toc" => ProgrammingLanguage::Tex,
            "txt" => ProgrammingLanguage::Text,
            "v" | "sv" => ProgrammingLanguage::V,
            "vala" | "vapi" => ProgrammingLanguage::Vala,
            "vb" | "bas" => ProgrammingLanguage::Vbnet,
            "vm" | "fhtml" => ProgrammingLanguage::Velocity,
            "vim" | "vimrc" => ProgrammingLanguage::Vim,
            "xml" | "rss" | "xsd" | "wsdl" => ProgrammingLanguage::Xml,
            "xqy" | "xquery" => ProgrammingLanguage::Xquery,
            "xsl" | "xslt" => ProgrammingLanguage::Xslt,
            "yaml" | "yml" => ProgrammingLanguage::Yaml,
            _ => ProgrammingLanguage::Unknown,
        }
    }
    pub fn generate_match_statement() {
        let mut statement = String::from(
            r#"
match obj {
"#,
        );
        for variant in Self::iter() {
            let variant: ProgrammingLanguage = variant;
            println!("Variant: {variant:?}");
            let strings = variant.to_extension();
            println!("Extensions: {strings:?}");
            statement.push_str("    ");
            strings.iter().for_each(|string| {
                statement.push_str(&format!("\"{string}\" | "));
            });
            statement = match statement.strip_suffix(" | ") {
                Some(stripped) => stripped.to_string(),
                None => statement,
            };
            statement.push_str(&format!(" => Some(ProgrammingLanguage::{variant:?}),\n"));
        }
        statement.push('}');
        println!("{}", statement);
    }
}
