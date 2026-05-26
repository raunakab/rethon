-- TODO:
-- 1. Define the `type` production.

pat ::=
  | $ident
  | $ident @ $pat
  | $pat | $pat
  | _
  
  | $literal
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
  | $ident $(: $type)? := $block
  | $(mut) $pat $(: $type)? = $block $(else $block)?

expr ::=
  -- expressions
  | $ident
  | $literal
  | ($($expr),+) -- tuples; there must be at least one `,` in there to explicitly inform the compiler that this must be treated as a tuple
  | [$($expr),*] -- lists
  | [[$($expr),*]] -- sets
  | {$($expr:$expr),*} -- maps
  
  -- logical-constructs
  | $if-else
  | $match
  | $loop
  
  -- functions
  | $function
  | $function-invocation
  | return $($block)?
  | yield $($block)?
  | throw $($block)?
  
  -- typedefs + type-invocations
  | $struct
  | $enum
  | $ident { $($ident $(: $expr)?)[,]* }
  
  -- impl-holes
  | panic
  | todo
  | unimplemented
  
  -- recursion
  | $expr: $type
  | $block

  -- macros

if-else ::=
  if ($block) $block
  $(else if ($block) $block)*
  $(else $block)?

match ::=
  match ($block)
    $($pat $(if $block)? => $block)[,]*

loop ::=
  | loop $block
  | loop ($block) $block
  | loop ($pat in $block) $block

function ::=
  fn ($($ident $(: $type)?),*) $(-> $type)? [:] $block
    
function-invocation ::=
  | $block($($block),*)
  | $block($($ident=$block),*)
  | $block($($block,)+ $($ident=$block),*)

struct ::=
  struct
    $($ident: $type)[;]*

enum ::=
  enum
    $($enum-variant)[|]*

enum-variant ::=
  | $ident
  | $ident($($type),*)
  | $ident{$($ident: $type),*}
