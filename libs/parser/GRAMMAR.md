-- TODO:
-- 1. Decide on set literal syntax (current `[{...}]` is visually ambiguous)
-- 2. Define the `type` production

pat ::=
  | $ident
  | $literal
  | $ident @ $pat
  | $pat | $pat
  | _

  | ($($pat),+)
  | [$($pat),*]
  | {$($literal:$pat),*}
  | $ident($($pat),*)
  | $ident { $($ident$(: $pat)?),* }

literal ::=
  | true
  | false
  | $number
  | $float
  | $string

type ::=
  -- etc.

block ::=
  $($item)[;]*

item ::=
  | $statement
  | $expr

statement ::=
  | $ident $(: $type)? := $expr
  | $(mut) $pat $(: $type)? = $expr $(else $expr)?

expr ::=
  -- expressions
  | $ident
  | $literal
  | ($($expr),$($expr),*) -- tuples; there must be at least one `,` in there to explicitly inform the compiler that this must be treated as a tuple
  | [$($expr),*] -- lists
  | [{$($expr),*}] -- sets
  | {$($expr:$expr),*} -- maps
  
  -- logical-constructs
  | $if-else
  | $match
  | $loop
  
  -- functions
  | $function
  | $function-invocation
  | return $($expr)?
  | yield $($expr)?
  | throw $($expr)?
  
  -- typedefs
  | $struct
  | $enum
  
  -- impl-holes
  | panic
  | todo
  | unimplemented
  
  -- recursion
  | $expr: $type
  | $block

if-else ::=
  if ($expr) $expr
  $(else if ($expr) $expr)*
  $(else $expr)?

match ::=
  match ($expr)
    $($pat $(if $expr)? => $expr)[;]*

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
  | $expr($($expr,)+ $($ident=$expr),*)

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
