#[macro_export]
macro_rules! analit (
    (                             +-- $($t:tt)+) => (analit!(X   1   ,    ,    ;$($t)+));
    (X   $x:expr,       ,       ; --  $($t:tt)+) => (analit!(X   $x+1,    ,    ;$($t)+));
    (X   $x:expr,       ,       ; +            ) => (($x));


    (                             +|  $($t:tt)+) => (analit!(Y   1   ,    ,    ;$($t)+));
    (Y   $x:expr,       ,       ; |   $($t:tt)+) => (analit!(Y   $x+1,    ,    ;$($t)+));
    (Y   $x:expr,       ,       ; +            ) => (($x));


    (                             +/  $($t:tt)+) => (analit!(Z   1   ,    ,    ;$($t)+));
    (Z   $x:expr,       ,       ; /   $($t:tt)+) => (analit!(Z   $x+1,    ,    ;$($t)+));
    (Z   $x:expr,       ,       ; +            ) => (($x));

    
    (X   $x:expr,       ,       ; +/  $($t:tt)+) => (analit!(XZ  $x  ,1   ,    ;$($t)+));
    (XZ  $x:expr,$y:expr,       ; /   $($t:tt)+) => (analit!(XZ  $x  ,$y+1,    ;$($t)+));
    (XZ  $x:expr,$y:expr,       ; +   $($t:tt)+) => (($x,$y/2));


    (X   $x:expr,       ,       ; +|  $($t:tt)+) => (analit!(XY  $x  ,1   ,    ;$($t)+));
    (XY  $x:expr,$y:expr,       ; |   $($t:tt)+) => (analit!(XY  $x  ,$y+1,    ;$($t)+));
    (XY  $x:expr,$y:expr,       ; +-  $($t:tt)+) => (($x,$y/2));


    (Z   $x:expr,       ,       ; |   $($t:tt)+) => (analit!(YZ  $x  ,1   ,    ;$($t)+));
    (YZ  $x:expr,$y:expr,       ; |   $($t:tt)+) => (analit!(YZ  $x  ,$y+1,    ;$($t)+));
    (YZ  $x:expr,$y:expr,       ; /   $($t:tt)+) => (analit!(YZ  $x+1,$y  ,    ;$($t)+));
    (YZ  $x:expr,$y:expr,       ; +   $($t:tt)+) => (analit!(YZ  $x  ,$y  ,    ;$($t)+));
    (YZ  $x:expr,$y:expr,       ; +            ) => (($x/2,$y/2));


    (XZ  $x:expr,$y:expr,       ; |   $($t:tt)+) => (analit!(XYZ $x  ,1   ,$y*2;$($t)+));
    (XYZ $x:expr,$y:expr,$z:expr; |   $($t:tt)+) => (analit!(XYZ $x  ,$y+1,$z  ;$($t)+));
    (XYZ $x:expr,$y:expr,$z:expr; /   $($t:tt)+) => (analit!(XYZ $x  ,$y  ,$z+1;$($t)+));
    (XYZ $x:expr,$y:expr,$z:expr; -   $($t:tt)+) => (analit!(XYZ $x  ,$y  ,$z  ;$($t)+));
    (XYZ $x:expr,$y:expr,$z:expr; +   $($t:tt)+) => (analit!(XYZ $x  ,$y  ,$z  ;$($t)+));
    (XYZ $x:expr,$y:expr,$z:expr; +            ) => (($x, $y/3, $z/3));
    
);

#[test]
fn one_dimension() {
    assert_eq!(2, analit!(
        +----+
    ));
    
    assert_eq!(2, analit!(
        +
        |
        |
        +
    ));
    
    assert_eq!(2,analit!(
           +
          /
         /
        +
    ));
}

#[test]
fn two_dimensional() {

    assert_eq!((2,1),analit!(
        +----+
        |    |
        +----+
    ));
    
    
    assert_eq!((2,1),analit!(
          +----+
         /    /
        +----+
    ));
    
    assert_eq!((2,1),analit!(
           +
          /|
         / +
        + /
        |/
        +
    ));
    
    assert_eq!((1,1),analit!(
           +
          /|
         + +
         |/
         +
    ));
    
    assert_eq!((1,2),analit!(
           +
          /|
         + |
         | +
         |/
         +
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

