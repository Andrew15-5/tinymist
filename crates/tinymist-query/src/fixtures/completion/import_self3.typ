/// path: base.typ
#let aa() = 1;
#let aab() = 1;
#let aac() = 1;
#let aabc() = 1;

-----
/// contains: base,baz,aa,aab,aac,aabc,aa.with,aa.where
#import "base.typ" as baz
#bac(/* range -2..0 */);