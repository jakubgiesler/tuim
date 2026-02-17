#[derive(Debug, Clone, Copy)]
pub enum Event {
    Character(char),
    Backspace,
    Down,
    Up,
}

pub type ShouldRedraw = bool;

pub const SEARCH_BOX_SIZE: usize = 28;
pub const STRING_SIZE: usize = 256;
pub const BUFFER_SIZE: usize = 256;

//

#[rustfmt::skip]
pub fn check_platform() {
    #[cfg(not(target_os = "linux"))]
    compile_error!(
r#"Non Linux Peasant

`;-.          ___,
  `.`\_...._/`.-"`
    \        /      ,
    /()   () \    .' `-._
   |)  .    ()\  /   _.'
   \  -'-     ,; '. <
    ;.__     ,;|   > \
   / ,    / ,  |.-'.-'
  (_/    (_/ ,;|.<`
    \    ,     ;-`
     >   \    /
    (_,-'`> .'
         (_,'
"#);

    {}
}
