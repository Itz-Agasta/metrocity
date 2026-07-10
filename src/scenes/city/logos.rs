use ratatui::style::Color;

#[derive(Clone)]
pub struct LogoPixel {
    pub ch: char,
    pub color: Color,
    pub bg: Color,
}

pub struct DistroLogo {
    pub grid: Vec<Vec<Option<LogoPixel>>>,
    pub is_compact: bool,
    pub display_name: String,
}

macro_rules! prepare_logo_grid {
    ($grid:expr, $lines:expr, $is_compact:expr, $b_bg:expr, $offset_y_expr:expr) => {{
        let mut min_x = 999;
        let mut max_x = 0;
        let mut min_y = 999;
        let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in $lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        let mut offset_x = 0;
        let mut offset_y = 0;

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            offset_x = (32_usize.saturating_sub(art_width)) / 2;
            offset_y = $offset_y_expr;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        $grid[gy][gx] = Some(LogoPixel {
                            ch: ' ',
                            color: $b_bg,
                            bg: $b_bg,
                        });
                    }
                }
            }
        }
        (points, min_x, min_y, offset_x, offset_y)
    }};
}

pub fn get_logo(distro: &str) -> DistroLogo {
    let d = distro.to_lowercase();
    let mut grid = vec![vec![None; 32]; 20];
    let b_bg = Color::Reset;
    let mut is_compact = false;
    #[allow(unused_assignments)]
    let mut display_name = "Linux".to_string();

    if d.contains("ubuntu") {
        is_compact = true;
        let white = Color::White;
        let orange = Color::Rgb(226, 88, 34);
        let lines = [
            "         _    ",
            "     ---(_)   ",
            " _/  ---  \\   ",
            "(_) |   |     ",
            "  \\  --- _/   ",
            "     ---(_)   ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '_' || ch == '-' || ch == '/' || ch == '\\' || ch == '|' {
                        white
                    } else {
                        orange
                    };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Ubuntu".to_string();
    } else if d.contains("debian") {
        is_compact = true;
        let red = Color::Rgb(215, 10, 83);
        let b_bg = Color::Reset;
        let lines = [
            "  _____     ",
            " /  __ \\    ",
            "|  /    |   ",
            "|  \\___-    ",
            " -_          ",
            "   --_       ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = red;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Debian".to_string();
    } else if d.contains("freebsd") {
        is_compact = true;
        let red = Color::Rgb(204, 0, 0);
        let white = Color::White;
        let b_bg = Color::Reset;

        let lines = [
            "/\\,-'''''-,/\\   ",
            "\\_)       (_/   ",
            "|           |   ",
            "|           |   ",
            " ;         ;    ",
            "  '-_____-'     ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = red;
                    if (y == 0 && (x <= 1 || x >= 11)) || (y == 1 && (x <= 2 || x >= 10)) {
                        color = white;
                    }

                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "FreeBSD".to_string();
    } else if d.contains("openbsd") {
        is_compact = true;
        let yellow = Color::Yellow;
        let white = Color::White;
        let b_bg = Color::Reset;

        let lines = [
            "      _____       ",
            "    \\-     -/     ",
            " \\_/         \\    ",
            " |        O O |   ",
            " |_  <   )  3 )   ",
            " / \\         /    ",
            "    /-_____-\\     ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = yellow;
                    if (y == 3 && (x == 10 || x == 12))
                        || (y == 4 && (x == 17 || x == 18 || x == 12 || x == 13 || x == 14))
                        || (y == 4 && ch == '<')
                        || (y == 4 && ch == ')')
                        || (y == 4 && ch == '3')
                        || ch == 'O'
                    {
                        color = white;
                    }

                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "OpenBSD".to_string();
    } else if d.contains("arch") {
        // I use Arch btw
        is_compact = true;
        let cyan = Color::Cyan;
        let b_bg = Color::Reset;
        let lines = [
            "       /\\       ",
            "      /  \\      ",
            "     /\\   \\     ",
            "    /      \\    ",
            "   /   ,,   \\   ",
            "  /   |  |   \\  ",
            " /_,,_    _,,_\\ ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = cyan;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "ARCH LINUX".to_string();
    } else if d.contains("windows") {
        is_compact = true;
        let blue = Color::Rgb(0, 180, 255);
        for (y, row) in grid.iter_mut().enumerate().take(14).skip(3) {
            for (x, cell) in row.iter_mut().enumerate().take(26).skip(6) {
                if x == 15 || y == 8 {
                    continue;
                }
                *cell = Some(LogoPixel {
                    ch: '█',
                    color: blue,
                    bg: b_bg,
                });
            }
        }
        display_name = "Windows".to_string();
    } else if d.contains("macos") || d.contains("apple") || d.contains("darwin") {
        is_compact = true;
        let b_bg = Color::Reset;
        let lines = [
            "       .:'     ",
            "    _ :'_      ",
            " .'`_`-'_``.   ",
            ":________.-'   ",
            ":_______:      ",
            ":_______:      ",
            " :_______`-;   ",
            "  `._.-._.'    ",
        ];

        let colors = [
            Color::Green,
            Color::Green,
            Color::Yellow,
            Color::Rgb(255, 165, 0),
            Color::Red,
            Color::Magenta,
            Color::Magenta,
            Color::Blue,
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = colors[y % colors.len()];
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "macOS".to_string();
    } else if d.contains("fedora") {
        is_compact = true;
        let blue = Color::Rgb(60, 110, 255);
        let white = Color::White;
        let b_bg = Color::Reset;
        let lines = [
            "      _____     ",
            "     /   __)\\    ",
            "     |  /  \\ \\   ",
            "  ___|  |__/ /   ",
            " / (_    _)_/    ",
            "/ /  |  |        ",
            "\\ \\__/  |        ",
            " \\(_____/        ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = match y {
                        0 => white,
                        1 => {
                            if x <= 11 {
                                white
                            } else {
                                blue
                            }
                        }
                        2 => {
                            if x <= 10 {
                                white
                            } else {
                                blue
                            }
                        }
                        3 => {
                            if (4..=5).contains(&x) || (8..=9).contains(&x) {
                                white
                            } else {
                                blue
                            }
                        }
                        4 => {
                            if (3..=10).contains(&x) {
                                white
                            } else {
                                blue
                            }
                        }
                        5 => {
                            if x >= 5 {
                                white
                            } else {
                                blue
                            }
                        }
                        6 => {
                            if (3..=5).contains(&x) || x >= 8 {
                                white
                            } else {
                                blue
                            }
                        }
                        7 => {
                            if x >= 2 {
                                white
                            } else {
                                blue
                            }
                        }
                        _ => white,
                    };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Fedora".to_string();
    } else if d.contains("nixos") {
        is_compact = true;
        let light_cyan = Color::Rgb(126, 186, 223);
        let dark_cyan = Color::Rgb(82, 119, 186);
        let b_bg = Color::Reset;
        let lines = vec![
            "             ___   __              ",
            "     /#\\     \\QQ\\ /fy;            ",
            "     \\#+\\     \\lQvfy/             ",
            "  ,=#####=##+\\ \\QOy/   /,         ",
            " /+=#######=++\\ \\Qq\\  /+#;        ",
            "      ,——,       \\O/ /+#/_        ",
            "_____/fy/         ‘ /+###+\\       ",
            "\\QOOQfy/           /##/¯¯¯¯       ",
            " ¯¯/fy/ ,         /y#/            ",
            "  ,fy/ /+\\  _____________        ",
            "   \\Y  \\##\\ \\QQqQeeOoQQQy\\       ",
            "       /#|#\\ ‾‾‾‾‾\\EQ\\‾‾‾‾       ",
            "      ,+#,\\#\\      \\QQ\\          ",
            "      \\#/ \\##\\      \\Q/          ",
            "       ‾   ‾‾‾                  ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = match y {
                        0 => light_cyan,
                        1 => {
                            if x <= 7 {
                                dark_cyan
                            } else {
                                light_cyan
                            }
                        }
                        2 => {
                            if x <= 8 {
                                dark_cyan
                            } else {
                                light_cyan
                            }
                        }
                        3 => {
                            if (x <= 13) || (22..=24).contains(&x) {
                                dark_cyan
                            } else {
                                light_cyan
                            }
                        }
                        4 => {
                            if (x <= 14) || (21..=25).contains(&x) {
                                dark_cyan
                            } else {
                                light_cyan
                            }
                        }
                        5 => {
                            if x <= 19 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        6 => {
                            if x <= 18 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        7 => {
                            if x <= 17 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        8 => {
                            if x <= 6 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        9 => {
                            if (x <= 5) || (x >= 10) {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        10 => {
                            if (x <= 5) || (x >= 11) {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        11 => {
                            if x >= 12 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        12 => {
                            if x >= 13 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        13 => {
                            if x >= 14 {
                                light_cyan
                            } else {
                                dark_cyan
                            }
                        }
                        _ => dark_cyan,
                    };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "NixOS".to_string();
    } else if d.contains("pop") {
        is_compact = true;
        let cyan = Color::Rgb(82, 187, 205);
        let b_bg = Color::Reset;
        let lines = [
            "______              ",
            "\\   _ \\        __   ",
            " \\ \\ \\ \\      / /   ",
            "  \\ \\_\\ \\    / /    ",
            "   \\  ___\\  /_/     ",
            "    \\ \\    _        ",
            "   __\\_\\__(_)_      ",
            "  (___________)`    ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = cyan;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Pop!_OS".to_string();
    } else if d.contains("kali") {
        let kali_blue = Color::Rgb(85, 155, 180);
        let b_bg = Color::Reset;
        let lines = vec![
            "  ÆÆÆ                        ",
            "          ÆÆÆÆÆÆ             ",
            "               Æ             ",
            "        ÆÆÆÆ    ÆÆ           ",
            "    ÆÆÆ        ÆÆÆÆÆÆÆÆÆÆ    ",
            "              ÆÆ       ÆÆÆ   ",
            "              ÆÆ          ÆÆ ",
            "              ÆÆÆÆ           ",
            "                 ÆÆÆÆÆÆÆ     ",
            "                       ÆÆÆ   ",
            "                          Æ  ",
            "                           Æ ",
            "                           ÆÆ",
        ];

        let mut min_x = 999;
        let mut max_x = 0;
        let mut min_y = 999;
        let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' && ch != '⠀' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = (20_usize.saturating_sub(art_height)) / 2;

            for (x, y, ch) in points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: kali_blue,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Kali Linux".to_string();
    } else if d.contains("gentoo") {
        is_compact = true;
        let purple = Color::Rgb(125, 115, 180);
        let white = Color::White;
        let b_bg = Color::Reset;
        let lines = [
            " _-----_ ",
            "(       \\",
            "\\    0   \\",
            " \\        )",
            " /      _/",
            "(     _- ",
            "\\____-   ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '0' { white } else { purple };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Gentoo".to_string();
    } else if d.contains("suse") {
        is_compact = true;
        let green = Color::Rgb(115, 186, 37);
        let b_bg = Color::Reset;
        let lines = [
            "  _______     ",
            "__|   __ \\    ",
            "     / .\\ \\   ",
            "     \\__/ |   ",
            "   _______|   ",
            "   \\_______   ",
            "__________/   ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "openSUSE".to_string();
    } else if d.contains("manjaro") {
        is_compact = true;
        let green = Color::Rgb(53, 191, 92);
        let b_bg = Color::Reset;
        let lines = [
            "||||||||| ||||   ",
            "||||||||| ||||   ",
            "||||      ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
            "|||| |||| ||||   ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Manjaro".to_string();
    } else if d.contains("void") {
        is_compact = true;
        let green = Color::Rgb(71, 128, 97);
        let b_bg = Color::Reset;
        let lines = [
            "    _______     ",
            " _ \\______ -    ",
            "| \\  ___  \\ |   ",
            "| | /   \\ | |   ",
            "| | \\___/ | |   ",
            "| \\______ \\_|   ",
            " -_______\\      ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = green;
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Void Linux".to_string();
    } else if d.contains("centos") {
        is_compact = true;
        let b_bg = Color::Reset;

        let lines = [
            " ____^____    ",
            " |\\  |  /|    ",
            " | \\ | / |    ",
            "<---- ---->   ",
            " | / | \\ |    ",
            " |/__|__\\|    ",
            "     v        ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    // Quadrant colors: green/yellow/cyan cross, orange arms
                    let color = if (y <= 2 && x <= 4) || ((4..=5).contains(&y) && x == 5) || y == 6
                    {
                        Color::Green
                    } else if (y <= 2 && x == 5) || ((4..=5).contains(&y) && x >= 6) {
                        Color::Yellow
                    } else if (y <= 2 && x >= 6) || (y == 3 && x <= 4) {
                        Color::Cyan
                    } else if (y == 3 && x >= 6) || ((4..=5).contains(&y) && x <= 4) {
                        Color::Rgb(255, 165, 0)
                    } else {
                        Color::White
                    };

                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "CentOS".to_string();
    } else if d.contains("mint") {
        is_compact = true;
        let green = Color::Rgb(141, 198, 63);
        let b_bg = Color::Reset;
        let lines = [
            " ___________ ",
            "|_          \\",
            "  | | _____ |",
            "  | | | | | |",
            "  | | | | | |",
            "  | \\_____/ |",
            "  \\_________/",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: green,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Linux Mint".to_string();
    } else if d.contains("android") {
        is_compact = true;
        let green = Color::Rgb(164, 198, 57);
        let b_bg = Color::Reset;
        let lines = [
            "  ;,           ,; ",
            "   ';,.-----.,;'  ",
            "  ,'           ', ",
            " /    O     O    \\",
            "|                 |",
            "'-----------------'",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: green,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Android".to_string();
    } else if d.contains("elementary") {
        is_compact = true;
        let blue = Color::Rgb(64, 150, 238);
        let b_bg = Color::Reset;
        let lines = [
            "  _______  ",
            " / ____  \\ ",
            "/  |  /  /\\",
            "|__\\ /  / |",
            "\\   /__/  /",
            " \\_______/ ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: blue,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "elementary OS".to_string();
    } else if d.contains("slackware") {
        is_compact = true;
        let blue = Color::Rgb(50, 100, 200);
        let b_bg = Color::Reset;
        let lines = [
            "   ________   ",
            "  /  ______|  ",
            "  | |______   ",
            "  \\______  \\  ",
            "   ______| |  ",
            "| |________/  ",
            "|____________ ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: blue,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Slackware".to_string();
    } else if d.contains("parrot") {
        is_compact = true;
        let cyan = Color::Rgb(0, 255, 255);
        let white = Color::White;
        let b_bg = Color::Reset;
        let lines = [
            "       _------.      ",
            "      /  ,     \\_    ",
            "    /   /  /{}\\ |o\\_ ",
            "   /    \\  `--' /-' \\",
            "  |      \\      \\    |",
            " |              |`-, |",
            " /              /__/)/",
            "|              |      ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if ch == '{'
                        || ch == '}'
                        || ch == 'o'
                        || ch == ','
                        || ch == '`'
                        || ch == '\''
                    {
                        white
                    } else {
                        cyan
                    };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Parrot OS".to_string();
    } else if d.contains("endeavour") {
        is_compact = true;
        let c1 = Color::Rgb(255, 74, 90);
        let c2 = Color::Rgb(125, 124, 196);
        let c3 = Color::Rgb(0, 180, 255);
        let b_bg = Color::Reset;
        let lines = [
            "           /*",
            "         #***** ",
            "       #********#",
            "     ##**********##",
            "   ##*************##",
            " ###**************##",
            "   ###############",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = c1;
                    match y {
                        0 => color = c1,
                        1 => {
                            if (10..=13).contains(&x) {
                                color = c2;
                            } else if x == 14 {
                                color = c3;
                            }
                        }
                        2 => {
                            if (8..=15).contains(&x) {
                                color = c2;
                            } else if x == 16 {
                                color = c3;
                            }
                        }
                        3 => {
                            if (7..=16).contains(&x) {
                                color = c2;
                            } else if x >= 17 {
                                color = c3;
                            }
                        }
                        4 => {
                            if (6..=17).contains(&x) {
                                color = c2;
                            } else if x >= 18 {
                                color = c3;
                            }
                        }
                        5 => {
                            if (5..=17).contains(&x) {
                                color = c2;
                            } else if x >= 18 {
                                color = c3;
                            }
                        }
                        6 => {
                            color = c3;
                        }
                        _ => {}
                    }
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "EndeavourOS".to_string();
    } else if d.contains("rhel") || d.contains("redhat") {
        is_compact = true;
        let red = Color::Rgb(255, 0, 0);
        let b_bg = Color::Reset;
        let lines = vec![
            "      .M.:MMM      ",
            "     MMMMMMMMMM.   ",
            "    ,MMMMMMMMMMM   ",
            " .MM MMMMMMMMMMM   ",
            "MMMM   MMMMMMMMM   ",
            "MMMMMM           MM",
            " MMMMMMMMM     ,MMMM",
            "   MMMMMMMMMMMMMMMM:",
            "      `MMMMMMMMMMMM",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: red,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Red Hat".to_string();
    } else if d.contains("zorin") {
        is_compact = true;
        let blue = Color::Rgb(0, 173, 239);
        let b_bg = Color::Reset;
        let lines = [
            "       ZZZZZZ",
            "           ZZ",
            "         ZZ  ",
            "       ZZ    ",
            "     ZZ      ",
            "    ZZZZZZ   ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: blue,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Zorin OS".to_string();
    } else if d.contains("garuda") {
        is_compact = true;
        let pink = Color::Rgb(255, 0, 255);
        let purple = Color::Rgb(128, 0, 255);
        let b_bg = Color::Reset;
        let lines = [
            "        .~~~~-.        ",
            "       /    ,__`)      ",
            "      |      \\o/|'-.   ",
            "      |         /  ,\\  ",
            "      |        ('--./  ",
            "      /         \\      ",
            "     /  ,  ,  ,  \\     ",
            "     `--'--'--'--'     ",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let color = if y <= 2 { pink } else { purple };
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Garuda Linux".to_string();
    } else if d.contains("artix") {
        is_compact = true;
        let cyan = Color::Cyan;
        let b_bg = Color::Reset;
        let lines = [
            "      /\\     ",
            "     /  \\    ",
            "    /\\   \\   ",
            "   /      \\  ",
            "  /   ,,   \\ ",
            " /   |  |  -\\",
            "/_-''    ''-_\\",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: cyan,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Artix Linux".to_string();
    } else if d.contains("cachy") {
        is_compact = true;
        let cyan = Color::Cyan;
        let b_bg = Color::Reset;
        let lines = vec![
            "     _____________    ",
            "    /            /   ◯",
            "   /    _______ /",
            "  /    /          ⟋ ⟍ ",
            " /    /           ⟍_⟋___",
            " \\    \\              /   \\",
            "  \\    \\_____________\\___/",
            "   \\                /",
            "    \\_____________ /",
        ];

        let mut min_x = 999;
        let mut max_x = 0;
        let mut min_y = 999;
        let mut max_y = 0;
        let mut points = Vec::new();

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let art_height = (max_y - min_y) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for by in 0..art_height {
                for bx in 0..art_width {
                    let gx = bx + offset_x;
                    let gy = by + offset_y;
                    if gx < 32 && gy < 20 {
                        grid[gy][gx] = Some(LogoPixel {
                            ch: ' ',
                            color: b_bg,
                            bg: b_bg,
                        });
                    }
                }
            }

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: cyan,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "CachyOS".to_string();
    } else if d.contains("emex") {
        is_compact = true;
        let white = Color::White;
        let b_bg = Color::Reset;
        let lines = vec![
            "          @@@@@@@@@@",
            "        @@@@@@@@@@@@@@",
            "      @@@@      @@@@",
            "    @@@@      @@@@",
            "  @@@@      @@@@",
            "@@@@      @@",
            "  @@@@",
            "    @@@@@@@@@@@@@@@@",
            "      @@@@@@@@@@@@@@",
        ];
        let (points, min_x, min_y, offset_x, offset_y) =
            prepare_logo_grid!(grid, lines, is_compact, b_bg, 2);
        if !points.is_empty() {
            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color: white,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Emex".to_string();
    } else {
        is_compact = true;
        let wireframe_body = Color::DarkGray;
        let white_eyes = Color::White;
        let neon_orange = Color::Rgb(255, 140, 0);
        let b_bg = Color::Reset;

        let lines = vec![
            "         _nnnn_",
            "        dGGGGMMb",
            "       @p~qp~~qMb",
            "       M|@||@) M|",
            "       @,----.JM|",
            "      JS^\\__/  qKL",
            "     dZP        qKRb",
            "    dZP          qKKb",
            "   fZP            SMMb",
            "   HZM            MMMM",
            "   FqM            MMMM",
            " __| \".        |\\dS\"qML",
            " |    `.       | `' \\Zq",
            "_)      \\.___.,|     .'",
            "\\____   )MMMMMP|   .'",
            "     `-'       `--'",
        ];

        let mut points = Vec::new();
        let mut min_x = 999;
        let mut max_x = 0;
        let mut min_y = 999;
        let mut max_y = 0;

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch != ' ' {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    points.push((x, y, ch));
                }
            }
        }

        if !points.is_empty() {
            let art_width = (max_x - min_x) + 1;
            let offset_x = (32_usize.saturating_sub(art_width)) / 2;
            let offset_y = 2;

            for &(x, y, ch) in &points {
                let gx = (x - min_x) + offset_x;
                let gy = (y - min_y) + offset_y;
                if gx < 32 && gy < 20 {
                    let mut color = wireframe_body;

                    if y == 3 && (8..=13).contains(&x) {
                        color = white_eyes;
                    } else if (y == 4 && (8..=13).contains(&x))
                        || (y == 5 && (8..=12).contains(&x))
                        || (y == 11 && ((1..=6).contains(&x) || (15..=16).contains(&x)))
                        || (y == 12 && ((1..=7).contains(&x) || (15..=18).contains(&x) || x == 20))
                        || y == 13
                        || (y == 14 && (x <= 8 || x >= 15))
                        || y == 15
                    {
                        color = neon_orange;
                    }

                    grid[gy][gx] = Some(LogoPixel {
                        ch,
                        color,
                        bg: b_bg,
                    });
                }
            }
        }
        display_name = "Linux".to_string();
    }

    DistroLogo {
        grid,
        is_compact,
        display_name,
    }
}
