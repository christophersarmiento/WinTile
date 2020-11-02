use std::fs;
use serde_json;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use winapi::shared::windef::HWND;
use hex;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
  gap: i32,
  edge_gap: i32,
  bindings: HashMap<String, String>,
}

fn parse_config(config_path : &str) -> Config {
  let mut config_file = fs::read_to_string(config_path).expect("Unable to read config file.");
  let config : Config = serde_json::from_str(config_file.as_mut_str()).expect("JSON was not well-formatted");

  return config;
}

#[cfg(windows)]
fn get_display_bounds( win : HWND) -> winapi::shared::windef::RECT {
  use winapi::um::winuser::{ MonitorFromWindow, GetMonitorInfoA, MONITORINFO, MONITOR_DEFAULTTONEAREST };
  use winapi::shared::windef::HMONITOR;

  let display : HMONITOR = unsafe {
    MonitorFromWindow(win, MONITOR_DEFAULTTONEAREST)
  };

  let mut info : MONITORINFO = unsafe { std::mem::zeroed() };
  info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
  
  unsafe {
    GetMonitorInfoA(display, &mut info)
  };

  return info.rcWork;
}

#[cfg(windows)]
fn tile_window(x:i32, y:i32, gap:i32, display_edge_gap:i32) {
  use winapi::um::winuser::{GetForegroundWindow, MoveWindow };

  let focused_window : HWND = unsafe {
    GetForegroundWindow()
  };

  let display_bounds = get_display_bounds(focused_window);

  let dx = display_bounds.left + x + display_edge_gap;
  let dy = display_bounds.top + y + display_edge_gap;
  let dw = display_bounds.right - display_edge_gap * 2;
  let dh = display_bounds.bottom - display_edge_gap * 2;

  // If we're moving into one of the horizontal edges, modify the window width
  let mut ww:i32 = dw;

  if x == -1 || x == 1 {
    ww = dw / 2;
  } 

  let mut wh:i32 = dh;

  if y == -1 || y == 1 {
    wh = dh / 2;
  }

  // Push the window in the direction of the tile, and cap to edge
  let mut wx:i32 = dx + x * dw;
  if wx < dx {
    wx = dx
  }
  else if wx + ww > dx + dw {
    wx = dx + dw - ww
  }
    
  let mut wy:i32 = dy + y * dh;
  if wy < dy{
    wy = dy
  }
  else if wy + wh > dy + dh {
    wy = dy + dh - wh
  }

  // Apply gaps
  wx = wx + gap;
  wy = wy + gap;
  ww = ww - gap * 2;
  wh = wh - gap * 2;

  unsafe {
    MoveWindow(focused_window, wx, wy, ww, wh, 1)
  };
}

fn tile(position:&str, config:&Config){
  match position {
    "TopLeft" =>  tile_window(-1,-1,config.gap,config.edge_gap),
    "Top" =>  tile_window(0,-1,config.gap,config.edge_gap),
    "TopRight" => tile_window(1,-1,config.gap,config.edge_gap),

    "Left" =>  tile_window(-1,0,config.gap,config.edge_gap),
    "Middle" =>  tile_window(0,0,config.gap,config.edge_gap),
    "Right" =>  tile_window(1,0,config.gap,config.edge_gap),

    "BottomLeft" =>  tile_window(-1,1,config.gap,config.edge_gap),
    "Bottom" =>  tile_window(0,1,config.gap,config.edge_gap),
    "BottomRight" =>  tile_window(1,1,config.gap,config.edge_gap),

    _ => tile_window(0,0,config.gap,config.edge_gap)
  };
}

fn keycode_to_string(k: u8) -> String {
  if (k >= 65 && k <= 90) || (k >= 48 && k <= 57) {
      return format!("{}", (k as char));
  }

  match k {
      0x01 => { "Mouseclick".to_string() }
      0x02 => { "Mouseclick".to_string() }
      0x03 => { "VK_CANCEL".to_string() }
      0x04 => { "Mouseclick".to_string() }
      0x05 => { "Mouseclick".to_string() }
      0x06 => { "Mouseclick".to_string() }
      0x08 => { "VK_BACK".to_string() }
      0x09 => { "VK_TAB".to_string() }
      0x0C => { "VK_CLEAR".to_string() }
      0x0D => { "VK_RETURN".to_string() }
      0x10 => { "VK_SHIFT".to_string() }
      0x11 => { "VK_CONTROL".to_string() }
      0x12 => { "VK_MENU".to_string() }
      0x13 => { "VK_PAUSE".to_string() }
      0x14 => { "VK_CAPITAL".to_string() }
      0x15 => { "VK_KANA,VK_HANGUEL,VK_HANGUL".to_string() }
      0x17 => { "VK_JUNJA".to_string() }
      0x18 => { "VK_FINAL".to_string() }
      0x19 => { "VK_HANJA,VK_KANJI".to_string() }
      0x1B => { "VK_ESCAPE".to_string() }
      0x1C => { "VK_CONVERT".to_string() }
      0x1D => { "VK_NONCONVERT".to_string() }
      0x1E => { "VK_ACCEPT".to_string() }
      0x1F => { "VK_MODECHANGE".to_string() }
      0x20 => { "VK_SPACE".to_string() }
      0x21 => { "VK_PRIOR".to_string() }
      0x22 => { "VK_NEXT".to_string() }
      0x23 => { "VK_END".to_string() }
      0x24 => { "VK_HOME".to_string() }
      0x25 => { "VK_LEFT".to_string() }
      0x26 => { "VK_UP".to_string() }
      0x27 => { "VK_RIGHT".to_string() }
      0x28 => { "VK_DOWN".to_string() }
      0x29 => { "VK_SELECT".to_string() }
      0x2A => { "VK_PRINT".to_string() }
      0x2B => { "VK_EXECUTE".to_string() }
      0x2C => { "VK_SNAPSHOT".to_string() }
      0x2D => { "VK_INSERT".to_string() }
      0x2E => { "VK_DELETE".to_string() }
      0x2F => { "VK_HELP".to_string() }
      0x5B => { "VK_LWIN".to_string() }
      0x5C => { "VK_RWIN".to_string() }
      0x5D => { "VK_APPS".to_string() }
      0x5F => { "VK_SLEEP".to_string() }
      0x60 => { "VK_NUMPAD0".to_string() }
      0x61 => { "VK_NUMPAD1".to_string() }
      0x62 => { "VK_NUMPAD2".to_string() }
      0x63 => { "VK_NUMPAD3".to_string() }
      0x64 => { "VK_NUMPAD4".to_string() }
      0x65 => { "VK_NUMPAD5".to_string() }
      0x66 => { "VK_NUMPAD6".to_string() }
      0x67 => { "VK_NUMPAD7".to_string() }
      0x68 => { "VK_NUMPAD8".to_string() }
      0x69 => { "VK_NUMPAD9".to_string() }
      0x6A => { "VK_MULTIPLY".to_string() }
      0x6B => { "VK_ADD".to_string() }
      0x6C => { "VK_SEPARATOR".to_string() }
      0x6D => { "VK_SUBTRACT".to_string() }
      0x6E => { "VK_DECIMAL".to_string() }
      0x6F => { "VK_DIVIDE".to_string() }
      0x70 => { "VK_F1".to_string() }
      0x71 => { "VK_F2".to_string() }
      0x72 => { "VK_F3".to_string() }
      0x73 => { "VK_F4".to_string() }
      0x74 => { "VK_F5".to_string() }
      0x75 => { "VK_F6".to_string() }
      0x76 => { "VK_F7".to_string() }
      0x77 => { "VK_F8".to_string() }
      0x78 => { "VK_F9".to_string() }
      0x79 => { "VK_F10".to_string() }
      0x7A => { "VK_F11".to_string() }
      0x7B => { "VK_F12".to_string() }
      0x7C => { "VK_F13".to_string() }
      0x7D => { "VK_F14".to_string() }
      0x7E => { "VK_F15".to_string() }
      0x7F => { "VK_F16".to_string() }
      0x80 => { "VK_F17".to_string() }
      0x81 => { "VK_F18".to_string() }
      0x82 => { "VK_F19".to_string() }
      0x83 => { "VK_F20".to_string() }
      0x84 => { "VK_F21".to_string() }
      0x85 => { "VK_F22".to_string() }
      0x86 => { "VK_F23".to_string() }
      0x87 => { "VK_F24".to_string() }
      0x90 => { "VK_NUMLOCK".to_string() }
      0x91 => { "VK_SCROLL".to_string() }
      0xA0 => { "VK_LSHIFT".to_string() }
      0xA1 => { "VK_RSHIFT".to_string() }
      0xA2 => { "VK_LCONTROL".to_string() }
      0xA3 => { "VK_RCONTROL".to_string() }
      0xA4 => { "VK_LMENU".to_string() }
      0xA5 => { "VK_RMENU".to_string() }
      0xA6 => { "VK_BROWSER_BACK".to_string() }
      0xA7 => { "VK_BROWSER_FORWARD".to_string() }
      0xA8 => { "VK_BROWSER_REFRESH".to_string() }
      0xA9 => { "VK_BROWSER_STOP".to_string() }
      0xAA => { "VK_BROWSER_SEARCH".to_string() }
      0xAB => { "VK_BROWSER_FAVORITES".to_string() }
      0xAC => { "VK_BROWSER_HOME".to_string() }
      0xAD => { "VK_VOLUME_MUTE".to_string() }
      0xAE => { "VK_VOLUME_DOWN".to_string() }
      0xAF => { "VK_VOLUME_UP".to_string() }
      0xB0 => { "VK_MEDIA_NEXT_TRACK".to_string() }
      0xB1 => { "VK_MEDIA_PREV_TRACK".to_string() }
      0xB2 => { "VK_MEDIA_STOP".to_string() }
      0xB3 => { "VK_MEDIA_PLAY_PAUSE".to_string() }
      0xB4 => { "VK_LAUNCH_MAIL".to_string() }
      0xB5 => { "VK_LAUNCH_MEDIA_SELECT".to_string() }
      0xB6 => { "VK_LAUNCH_APP1".to_string() }
      0xB7 => { "VK_LAUNCH_APP2".to_string() }
      0xBA => { "VK_OEM_1".to_string() }
      0xBB => { "VK_OEM_PLUS".to_string() }
      0xBC => { "VK_OEM_COMMA".to_string() }
      0xBD => { "VK_OEM_MINUS".to_string() }
      0xBE => { "VK_OEM_PERIOD".to_string() }
      0xBF => { "VK_OEM_2".to_string() }
      0xC0 => { "VK_OEM_3".to_string() }
      0xDB => { "VK_OEM_4".to_string() }
      0xDC => { "VK_OEM_5".to_string() }
      0xDD => { "VK_OEM_6".to_string() }
      0xDE => { "VK_OEM_7".to_string() }
      0xDF => { "VK_OEM_8".to_string() }
      0xE2 => { "VK_OEM_102".to_string() }
      0xE5 => { "VK_PROCESSKEY".to_string() }
      0xF6 => { "VK_ATTN".to_string() }
      0xF7 => { "VK_CRSEL".to_string() }
      0xF8 => { "VK_EXSEL".to_string() }
      0xF9 => { "VK_EREOF".to_string() }
      0xFA => { "VK_PLAY".to_string() }
      0xFB => { "VK_ZOOM".to_string() }
      0xFC => { "VK_NONAME".to_string() }
      0xFD => { "VK_PA1".to_string() }
      0xFE => { "VK_OEM_CLEAR".to_string() }

      _ => { return format!("CODE_{}", k); }
  }
}

fn main() {
  let config = parse_config("./config.json");

  use winapi::um::winuser::{ RegisterHotKey, GetMessageW };
  
  let modifier = winapi::um::winuser::MOD_ALT as u32;
  let period = winapi::um::winuser::VK_OEM_PERIOD as u32;
  let comma = winapi::um::winuser::VK_OEM_COMMA as u32;
  let mut message : winapi::um::winuser::MSG = unsafe { std::mem::zeroed() };
  
  unsafe{
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("tl"), 16).unwrap(), modifier, 0x55); // u
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("tm"), 16).unwrap(), modifier, 0x49); // i
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("tr"), 16).unwrap(), modifier, 0x4f); // o

    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("ml"), 16).unwrap(), modifier, 0x4a); // j
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("mm"), 16).unwrap(), modifier, 0x4b); // k
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("mr"), 16).unwrap(), modifier, 0x4c); // l

    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("bl"), 16).unwrap(), modifier, 0x4D); // m
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("bm"), 16).unwrap(), modifier, comma); // ,
    RegisterHotKey(std::ptr::null_mut(), i32::from_str_radix(&hex::encode("br"), 16).unwrap(), modifier, period); // .
  };

  let mut result;

  while { result = unsafe { GetMessageW( &mut message, std::ptr::null_mut() , 0, 0 )} ; result != 0 } {
    let decode = &hex::decode(format!("{:X}", message.wParam as i32)).unwrap();
    let position = str::from_utf8(decode).unwrap();
    
    match position {
      "tl" => tile("TopLeft", &config),
      "tm" => tile("Top", &config),
      "tr" => tile("TopRight", &config),

      "ml" => tile("Left", &config),
      "mm" => tile("Middle", &config),
      "mr" => tile("Right", &config),

      "bl" => tile("BottomLeft", &config),
      "bm" => tile("Bottom", &config),
      "br" => tile("BottomRight", &config),
      
      _ => tile("Middle", &config)
    };
  }
}