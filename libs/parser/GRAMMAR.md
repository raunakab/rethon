pat ::=
  | $ident
  -- etc.

block ::=
  $($item)[;]*

item ::=
  | $statement
  | $expr

statement ::=
  $(mut) $pat $(: $type)? = $expr $(else $expr)?

expr ::=
  | $expr: $type

  -- expressions
  | $ident
  | true
  | false
  | $number
  | $float
  | {$($expr:$expr),*} -- maps
  | [$($expr),*] -- sets
  | ($(expr),$($expr),*) -- tuples; there must be at least one `,` in there to explicitly inform the compiler that this must be treated as a tuple
  
  -- logical-constructs
  | $if-else
  | $match
  | $loop
  
  -- functions
  | $function
  | $function-invocation
  
  -- typedefs
  | $struct
  | $enum
  
  -- impl-holes
  | panic
  | todo
  | unimplemented

if-else ::=
  if ($expr) $expr
  $(else if ($expr) $expr)*
  $(else $expr)?

match ::=
  match ($expr)
    $($pat $(if $expr)? => $expr),*

loop ::=
  | loop $expr
  | loop ($expr) $expr
  | loop ($pat in $expr) $expr

function ::=
  fn ($($ident $(: $type)?),*) $(-> $type)?
    $expr
    
function-invocation ::=
  | $expr($($expr),*)
  | $expr($($ident=$expr),*)
  | $expr($($expr,)+ $($($ident=$expr),*)?)

struct ::=
  struct
    $($ident: $type)[;]*

enum ::=
  enum
    $($enum-variant)[|]*

enum-variant ::=
  | $ident
  | $ident($type)
  | $ident{$($ident: $type),*}
