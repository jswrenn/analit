
#[macro_export]
macro_rules! analit {
    // Starting condition
    (+$($t:tt)+) => {analit!(0,$($t)+)};
    
    /* ONE DIMENSIONAL */
    ($dx:expr, +)               => {($dx/2)};
    ($dx:expr, -     $($t:tt)+) => {analit!(1+$dx,$($t)+)};
    
    // Depending on what character is present after the '+', we can
    // change our parsing technique to target 2D or 3D literals.
    ($dx:expr, + | | $($t:tt)+) => {analit!($dx,1,$($t)+)};
    ($dx:expr, + / / $($t:tt)+) => {analit!($dx,0,2,$($t)+)};
    
    /* TWO DIMENSIONAL */
    ($dx:expr, $dy:expr, +   $($t:tt)*)  => {($dx/2,$dy)};
    // Count how many pairs of pipes there are
    ($dx:expr, $dy:expr, | | $($t:tt)+) => {analit!($dx,$dy+1,$($t)+)};
    
    /* THREE DIMENSIONAL */
    ($dx:expr, $dy:expr, $dz:expr, | $($t:tt)+) => {analit!($dx,$dy+1,$dz,$($t)+)};
    ($dx:expr, $dy:expr, $dz:expr, / $($t:tt)+) => {analit!($dx,$dy,$dz+1,$($t)+)};
    ($dx:expr, $dy:expr, $dz:expr, + $($t:tt)+) => {analit!($dx,$dy,$dz,$($t)+)};
    ($dx:expr, $dy:expr, $dz:expr, - $($t:tt)+) => {analit!($dx,$dy,$dz,$($t)+)};
    ($dx:expr, $dy:expr, $dz:expr, +)           => {($dx/2,$dy/3,$dz/3)};
}

#[test]
fn one_dimension() {
    
    assert_eq!(0, analit!(
        +-+
    ));
    
    assert_eq!(1, analit!(
        +--+
    ));
}

#[test]
fn two_dimensional() {

    assert_eq!((1,1),analit!(
        +--+
        |  |
        +--+
    ));
    
    assert_eq!((8,1),analit!(
        +----------------+
        |                |
        +----------------+
    ));
}

#[test]
fn three_dimensional() {

    assert_eq!((1,1,3),analit!(
            +--+
           /  /|
          /  / +
         /  / /
        +--+ /
        |  |/
        +--+
    ));
        
    assert_eq!((4,1,1),analit!(
          +--------+
         /        /|
        +--------+ +
        |        |/
        +--------+
    ));

}

