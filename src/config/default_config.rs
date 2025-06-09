pub const DEFAULT_CONFIG_TOML: &str = r###"
timeout_ms = 1200

[window]
monitor = 0
position = [500.0, 500.0]
size = [800, 120]

[styles.normal]
width = 90.0
height = 90.0
icon_size = 0.0
text_size = 20.0
bg_color = "#1e1e30"
fg_color = "#ffffff"

[styles.modifier]
width = 120.0
height = 90.0
icon_size = 25.0
text_size = 18.0
bg_color = "#32283c"
fg_color = "#ffffff"

[styles.editor]
width = 90.0
height = 90.0
icon_size = 18.0
text_size = 22.0
bg_color = "#3f2e2e"
fg_color = "#ffffff"

[styles.navigation]
width = 90.0
height = 90.0
icon_size = 20.0
text_size = 22.0
bg_color = "#2e3f2e"
fg_color = "#ffffff"

[styles.scrollable]
width = 90.0
height = 90.0
icon_size = 20.0
text_size = 22.0
bg_color = "#2e3f2e"
fg_color = "#ffffff"

[styles.numeric]
width = 90.0
height = 90.0
icon_size = 0.0
text_size = 24.0
bg_color = "#2e2e2e"
fg_color = "#ffffff"

[styles.symbol]
width = 90.0
height = 90.0
icon_size = 20.0
text_size = 24.0
bg_color = "#3c2e2e"
fg_color = "#ffffff"

[styles.space]
width = 260.0
height = 90.0
icon_size = 20.0
text_size = 20.0 
bg_color = "#888888"
fg_color = "#ffffff"

[styles.escape]
width = 90.0
height = 90.0
icon_size = 20.0
text_size = 22.0
bg_color = "#AA1111"
fg_color = "#ffffff"

[styles.unknown]
width = 90.0
height = 90.0
icon_size = 14.0
text_size = 22.0
bg_color = "#555555"
fg_color = "#ffffff"

[styles.function]
width = 90.0
height = 90.0
icon_size = 14.0
text_size = 22.0
bg_color = "#001155"
fg_color = "#ffffff"

[styles.altfunction]
width = 90.0
height = 90.0
icon_size = 14.0
text_size = 22.0
bg_color = "#004488"
fg_color = "#ffffff"

[styles.mouse]
width = 90.0
height = 90.0
icon_size = 14.0
text_size = 22.0
bg_color = "#801155"
fg_color = "#ffffff"
"###;
