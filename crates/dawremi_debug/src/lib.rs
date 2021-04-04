use dawremi_core::prelude::Frame;

use open;
use std::fs::File;
use std::io::prelude::Write;
use std::path::Path;

/// Writes an html file with a visualizer for the provided slice
/// If open_in_browser is true, the default browser will be opened automatically with the file
///
/// The [open](https://docs.rs/open) crate is used to open the file in your browser.
/// If you have any issue with opening the file automatically, please report it directly there
pub fn debug_frames_vec_web(vec: &[Frame], filename: &str, open_in_browser: bool) {
    let string = format!("assets/debug/{}.html", filename);
    let path = Path::new(&string);

    let mut file = File::create(&path).unwrap();

    let mut string = String::from(
        r#"<html>
  <head>
      <meta charset="utf-8" />
      <title>Dawremi debug</title>

      <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/dygraph/2.1.0/dygraph.min.css" />
  </head>

  <body>
      <p id="loading">
          Loading, please wait
      </p>
      <p>Left channel</p>
      <div id="graph_div_left" style="width: 100%; height: 40vh;"></div><br>
      <p>Right channel</p>
      <div id="graph_div_right" style="width: 100%; height: 40vh;"></div><br>
      <p>Select area to zoom in</p>
      <p>Double click chart to unzoom</p>
        <p>
            Synchronize what?
            <input type="checkbox" id="chk-zoom" checked=""><label for="chk-zoom"> Zoom</label>
            <input type="checkbox" id="chk-selection" checked=""><label for="chk-selection"> Selection</label>
        </p>

      <script src="https://cdnjs.cloudflare.com/ajax/libs/dygraph/2.1.0/dygraph.min.js"></script>
      <script type="text/javascript" src="https://rawgit.com/danvk/dygraphs/e9155b2c442bf69396d5a456c885f938d38d35da/src/extras/synchronizer.js"></script>
      <script type="text/javascript">
          var left = new Dygraph(document.getElementById("graph_div_left"), [
"#,
    );
    for (i, el) in vec.iter().enumerate() {
        string.push_str(&format!("[{},{}],\n", i, el.left));
    }
    string.push_str(
        r#"
          ]);
          var right = new Dygraph(document.getElementById("graph_div_right"), [
"#,
    );
    for (i, el) in vec.iter().enumerate() {
        string.push_str(&format!("[{},{}],\n", i, el.right));
    }
    string.push_str(
        r#"
          ]);
          right.ready(function() {
              document.getElementById("loading").style.display = "none";
          });
          left.ready(function() {
              document.getElementById("loading").style.display = "none";
          });

          var gs = [left, right];

          var sync = Dygraph.synchronize(gs);

          function update() {
                var zoom = document.getElementById('chk-zoom').checked;
                var selection = document.getElementById('chk-selection').checked;
                sync.detach();
                sync = Dygraph.synchronize(gs, {
                    zoom: zoom,
                    selection: selection
                });
            }

           document.getElementById('chk-zoom').onchange = update;
           document.getElementById('chk-selection').onchange = update;
      </script>
  </body>
</html>
"#,
    );

    // Ignore errors
    let _ = file.write_all(string.as_bytes());

    if open_in_browser {
        let _ = open::that(path.display().to_string());
    }
}

/// Writes an html file with a visualizer for the provided slice
/// If open_in_browser is true, the default browser will be opened automatically with the file
///
/// The [open](https://docs.rs/open) crate is used to open the file in your browser.
/// If you have any issue with opening the file automatically, please report it directly there
pub fn debug_vec_web(vec: &[f64], filename: &str, open_in_browser: bool) {
    let string = format!("assets/debug/{}.html", filename);
    let path = Path::new(&string);

    let mut file = File::create(&path).unwrap();

    let mut string = String::from(
        r#"<html>
  <head>
      <meta charset="utf-8" />
      <title>Dawremi debug</title>

      <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/dygraph/2.1.0/dygraph.min.css" />
  </head>

  <body>
      <p id="loading">
          Loading, please wait
      </p>
      <div id="graph_div" style="width: 100%; height: 60vh;"></div><br>
      <p>Select area to zoom in</p>
      <p>Double click chart to unzoom</p>

      <script src="https://cdnjs.cloudflare.com/ajax/libs/dygraph/2.1.0/dygraph.min.js"></script>
      <script type="text/javascript">
          var g = new Dygraph(document.getElementById("graph_div"), [
"#,
    );
    for (i, el) in vec.iter().enumerate() {
        string.push_str(&format!("[{},{}],\n", i, el));
    }
    string.push_str(
        r#"
          ]);
          g.ready(function() {
              document.getElementById("loading").style.display = "none";
          });
      </script>
  </body>
</html>
"#,
    );

    // Ignore errors
    let _ = file.write_all(string.as_bytes());

    if open_in_browser {
        let _ = open::that(path.display().to_string());
    }
}
