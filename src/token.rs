use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
  val: String,
  t_type: TokenType,
  start: usize,
  end: usize,
  line: usize,
}

impl Token {
  pub fn new(val: String, t: TokenType, start: usize, end: usize, line: usize) -> Token {
    Token {
      val,
      t_type: t,
      start,
      end,
      line,
    }
  }

  pub fn get_type(&self) -> &TokenType {
    &self.t_type
  }

  pub fn get_value(&self) -> &String {
    &self.val
  }

  pub fn _get_start(&self) -> &usize {
    &self.start
  }

  pub fn _get_end(&self) -> &usize {
    &self.end
  }

  pub fn get_line(&self) -> &usize {
    &self.line
  }

  fn get_type_padding(&self) -> String {
    let max_len = 25;
    let my_len = format!("{:?}", self.t_type).len();
    self.get_padding(max_len - my_len)
  }

  fn get_text_padding(&self) -> String {
    let max_len = 25;
    let my_len = format!("{:?}", self.val).len();
    self.get_padding(max_len - my_len)
  }

  fn get_start_padding(&self) -> String {
    let max_len = 6;
    let my_len = format!("{:?}", self.start).len();
    self.get_padding(max_len - my_len)
  }

  fn get_end_padding(&self) -> String {
    let max_len = 6;
    let my_len = format!("{:?}", self.end).len();
    self.get_padding(max_len - my_len)
  }

  fn get_padding(&self, len: usize) -> String {
    let mut padding = String::new();
    for _ in 0..len {
      padding.push(' ');
    }
    padding
  }

  pub fn is_prec_level_seven(&self) -> bool {
    self.get_type() == &TokenType::BoolNot
  }

  pub fn is_prec_level_six(&self) -> bool {
    self.get_type() == &TokenType::BoolOr
  }

  pub fn is_prec_level_five(&self) -> bool {
    self.get_type() == &TokenType::BoolXor || self.get_type() == &TokenType::BoolAnd
  }

  pub fn is_prec_level_four(&self) -> bool {
    self.get_type() == &TokenType::Equal
      || self.get_type() == &TokenType::NotEqual
      || self.get_type() == &TokenType::LessThan
      || self.get_type() == &TokenType::GreaterThan
      || self.get_type() == &TokenType::LessThanOrEqual
      || self.get_type() == &TokenType::GreaterThanOrEqual
  }

  pub fn is_prec_level_three(&self) -> bool {
    self.get_type() == &TokenType::Addition
      || self.get_type() == &TokenType::Subtraction
      || self.get_type() == &TokenType::Or
  }

  pub fn is_prec_level_two(&self) -> bool {
    self.get_type() == &TokenType::Multiplication
      || self.get_type() == &TokenType::Division
      || self.get_type() == &TokenType::Shl
      || self.get_type() == &TokenType::Shr
      || self.get_type() == &TokenType::Xor
      || self.get_type() == &TokenType::And
      || self.get_type() == &TokenType::Modulo
  }

  pub fn is_prec_level_one(&self) -> bool {
    self.get_type() == &TokenType::Xor
      || self.get_type() == &TokenType::GreaterThan
      || self.get_type() == &TokenType::LessThan
      || self.get_type() == &TokenType::Not
      || self.get_type() == &TokenType::Addition
      || self.get_type() == &TokenType::Subtraction
      || self.is_built_in_pseudo_var()
      || self.is_built_in_pseudo_fun()
  }

  pub fn is_prec_level_zero(&self) -> bool {
    self.is_built_in_string_fun()
  }

  fn is_built_in_pseudo_var(&self) -> bool {
    self.get_type() == &TokenType::Star
      || self.get_type() == &TokenType::Asize
      || self.get_type() == &TokenType::Cpu
      || self.get_type() == &TokenType::Isize
      || self.get_type() == &TokenType::Paramcount
      || self.get_type() == &TokenType::Time
      || self.get_type() == &TokenType::Version
  }

  fn is_built_in_pseudo_fun(&self) -> bool {
    self.get_type() == &TokenType::Addrsize
      || self.get_type() == &TokenType::Bank
      || self.get_type() == &TokenType::Bankbyte
      || self.get_type() == &TokenType::Blank
      || self.get_type() == &TokenType::Const
      || self.get_type() == &TokenType::Hibyte
      || self.get_type() == &TokenType::Hiword
      || self.get_type() == &TokenType::Ident
      || self.get_type() == &TokenType::Left
      || self.get_type() == &TokenType::Lobyte
      || self.get_type() == &TokenType::Loword
      || self.get_type() == &TokenType::Match
      || self.get_type() == &TokenType::Max
      || self.get_type() == &TokenType::Mid
      || self.get_type() == &TokenType::Min
      || self.get_type() == &TokenType::Ref
      || self.get_type() == &TokenType::Referenced
      || self.get_type() == &TokenType::Right
      || self.get_type() == &TokenType::Sizeof
      || self.get_type() == &TokenType::Strat
      || self.get_type() == &TokenType::Sprintf
      || self.get_type() == &TokenType::String
      || self.get_type() == &TokenType::Strlen
      || self.get_type() == &TokenType::Tcount
      || self.get_type() == &TokenType::Xmatch
  }

  fn is_built_in_string_fun(&self) -> bool {
    self.get_type() == &TokenType::Concat
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let type_padding = self.get_type_padding();
    let text_padding = self.get_text_padding();
    let start_padding = self.get_start_padding();
    let end_padding = self.get_end_padding();
    write!(
      f,
      "type:\t{:?},{}text:\t{},{}start:\t{},{}end:\t{},{}line:\t{}",
      self.t_type,
      type_padding,
      self.val,
      text_padding,
      self.start,
      start_padding,
      self.end,
      end_padding,
      self.line
    )
  }
}

impl PartialOrd for Token {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Token {
  fn cmp(&self, other: &Self) -> Ordering {
    self.start.cmp(&other.start)
  }
}

impl Eq for Token {}

impl PartialEq for Token {
  fn eq(&self, other: &Self) -> bool {
    self.start == other.start && self.val == other.val
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
  Identifier,
  HexNumber,
  BinNumber,
  DecNumber,
  Semicolon,
  DirectiveA16,
  DirectiveA8,
  DirectiveAddr,
  DirectiveAddrsize,
  DirectiveAlign,
  DirectiveAnd,
  DirectiveAsciiz,
  DirectiveAsize,
  DirectiveAssert,
  DirectiveAutoimport,
  DirectiveBank,
  DirectiveBankbyte,
  DirectiveBankbytes,
  DirectiveBitand,
  DirectiveBitnot,
  DirectiveBitor,
  DirectiveBitxor,
  DirectiveBlank,
  DirectiveBss,
  DirectiveByt,
  DirectiveByte,
  DirectiveCase,
  DirectiveCharmap,
  DirectiveCode,
  DirectiveConcat,
  DirectiveCondes,
  DirectiveConst,
  DirectiveConstructor,
  DirectiveCpu,
  DirectiveData,
  DirectiveDbg,
  DirectiveDbyt,
  DirectiveDebuginfo,
  DirectiveDef,
  DirectiveDefine,
  DirectiveDefined,
  DirectiveDefinedmacro,
  DirectiveDelmac,
  DirectiveDelmacro,
  DirectiveDestructor,
  DirectiveDword,
  DirectiveElse,
  DirectiveElseif,
  DirectiveEnd,
  DirectiveEndenum,
  DirectiveEndif,
  DirectiveEndmac,
  DirectiveEndmacro,
  DirectiveEndproc,
  DirectiveEndrep,
  DirectiveEndrepeat,
  DirectiveEndscope,
  DirectiveEndstruct,
  DirectiveEndunion,
  DirectiveEnum,
  DirectiveError,
  DirectiveExitmac,
  DirectiveExitmacro,
  DirectiveExport,
  DirectiveExportzp,
  DirectiveFaraddr,
  DirectiveFatal,
  DirectiveFeature,
  DirectiveFileopt,
  DirectiveFopt,
  DirectiveForceimport,
  DirectiveForceword,
  DirectiveGlobal,
  DirectiveGlobalzp,
  DirectiveHibyte,
  DirectiveHibytes,
  DirectiveHiword,
  DirectiveI16,
  DirectiveI8,
  DirectiveIdent,
  DirectiveIf,
  DirectiveIfblank,
  DirectiveIfconst,
  DirectiveIfdef,
  DirectiveIfnblank,
  DirectiveIfnconst,
  DirectiveIfndef,
  DirectiveIfnref,
  DirectiveIfp02,
  DirectiveIfp4510,
  DirectiveIfp816,
  DirectiveIfpc02,
  DirectiveIfpsc02,
  DirectiveIfref,
  DirectiveImport,
  DirectiveImportzp,
  DirectiveIncbin,
  DirectiveInclude,
  DirectiveInterruptor,
  DirectiveIsize,
  DirectiveIsmnem,
  DirectiveIsmnemonic,
  DirectiveLeft,
  DirectiveLinecont,
  DirectiveList,
  DirectiveListbytes,
  DirectiveLobyte,
  DirectiveLobytes,
  DirectiveLocal,
  DirectiveLocalchar,
  DirectiveLoword,
  DirectiveMac,
  DirectiveMacpack,
  DirectiveMacro,
  DirectiveMatch,
  DirectiveMax,
  DirectiveMid,
  DirectiveMin,
  DirectiveMod,
  DirectiveNot,
  DirectiveNull,
  DirectiveOr,
  DirectiveOrg,
  DirectiveOut,
  DirectiveP02,
  DirectiveP4510,
  DirectiveP816,
  DirectivePagelen,
  DirectivePagelength,
  DirectiveParamcount,
  DirectivePc02,
  DirectivePopcpu,
  DirectivePopseg,
  DirectiveProc,
  DirectivePsc02,
  DirectivePushcpu,
  DirectivePushseg,
  DirectiveRef,
  DirectiveReferenced,
  DirectiveReloc,
  DirectiveRepeat,
  DirectiveRes,
  DirectiveRight,
  DirectiveRodata,
  DirectiveScope,
  DirectiveSegment,
  DirectiveSet,
  DirectiveSetcpu,
  DirectiveShl,
  DirectiveShr,
  DirectiveSizeof,
  DirectiveSmart,
  DirectiveSprintf,
  DirectiveStrat,
  DirectiveString,
  DirectiveStrlen,
  DirectiveStruct,
  DirectiveTag,
  DirectiveTcount,
  DirectiveTime,
  DirectiveUndef,
  DirectiveUndefine,
  DirectiveUnion,
  DirectiveVersion,
  DirectiveWarning,
  DirectiveWord,
  DirectiveXmatch,
  DirectiveXor,
  DirectiveZeropage,
  LocalLabel,
  XRegister,
  YRegister,
  Comment,
  Whitespace,
  Addition,
  Subtraction,
  Multiplication,
  Division,
  Assignment,
  Equal,
  NotEqual,
  Xor,
  Or,
  And,
  Comma,
  Colon,
  Hash,
  OParen,
  CParen,
  OCurly,
  CCurly,
  OBracket,
  CBracket,
  LessThan,
  LessThanOrEqual,
  GreaterThan,
  GreaterThanOrEqual,
  Not,
  Shl,
  Shr,
  BoolNot,
  BoolAnd,
  BoolOr,
  BoolXor,
  Modulo,
  StringConst,
  Newline,
  Namespace,
  ULabel,
  Opcode,
  Star,
  Asize,
  Cpu,
  Isize,
  Paramcount,
  Time,
  Version,
  Addrsize,
  Bank,
  Bankbyte,
  Blank,
  Const,
  Hibyte,
  Hiword,
  Ident,
  Left,
  Lobyte,
  Loword,
  Match,
  Max,
  Mid,
  Min,
  Ref,
  Referenced,
  Right,
  Sizeof,
  Strat,
  Sprintf,
  String,
  Strlen,
  Tcount,
  Xmatch,
  Concat,
  EndOfFile,
}

impl TokenType {
  pub fn get_directive_type(identifier: &str) -> TokenType {
    match identifier {
      "a16" => TokenType::DirectiveA16,
      "a8" => TokenType::DirectiveA8,
      "addr" => TokenType::DirectiveAddr,
      "addrsize" => TokenType::DirectiveAddrsize,
      "align" => TokenType::DirectiveAlign,
      "and" => TokenType::DirectiveAnd,
      "asciiz" => TokenType::DirectiveAsciiz,
      "asize" => TokenType::DirectiveAsize,
      "assert" => TokenType::DirectiveAssert,
      "autoimport" => TokenType::DirectiveAutoimport,
      "bank" => TokenType::DirectiveBank,
      "bankbyte" => TokenType::DirectiveBankbyte,
      "bankbytes" => TokenType::DirectiveBankbytes,
      "bitand" => TokenType::DirectiveBitand,
      "bitnot" => TokenType::DirectiveBitnot,
      "bitor" => TokenType::DirectiveBitor,
      "bitxor" => TokenType::DirectiveBitxor,
      "blank" => TokenType::DirectiveBlank,
      "bss" => TokenType::DirectiveBss,
      "byt" => TokenType::DirectiveByt,
      "byte" => TokenType::DirectiveByte,
      "case" => TokenType::DirectiveCase,
      "charmap" => TokenType::DirectiveCharmap,
      "code" => TokenType::DirectiveCode,
      "concat" => TokenType::DirectiveConcat,
      "condes" => TokenType::DirectiveCondes,
      "const" => TokenType::DirectiveConst,
      "constructor" => TokenType::DirectiveConstructor,
      "cpu" => TokenType::DirectiveCpu,
      "data" => TokenType::DirectiveData,
      "dbg" => TokenType::DirectiveDbg,
      "dbyt" => TokenType::DirectiveDbyt,
      "debuginfo" => TokenType::DirectiveDebuginfo,
      "def" => TokenType::DirectiveDef,
      "define" => TokenType::DirectiveDefine,
      "defined" => TokenType::DirectiveDefined,
      "definedmacro" => TokenType::DirectiveDefinedmacro,
      "delmac" => TokenType::DirectiveDelmac,
      "delmacro" => TokenType::DirectiveDelmacro,
      "destructor" => TokenType::DirectiveDestructor,
      "dword" => TokenType::DirectiveDword,
      "else" => TokenType::DirectiveElse,
      "elseif" => TokenType::DirectiveElseif,
      "end" => TokenType::DirectiveEnd,
      "endenum" => TokenType::DirectiveEndenum,
      "endif" => TokenType::DirectiveEndif,
      "endmac" => TokenType::DirectiveEndmac,
      "endmacro" => TokenType::DirectiveEndmacro,
      "endproc" => TokenType::DirectiveEndproc,
      "endrep" => TokenType::DirectiveEndrep,
      "endrepeat" => TokenType::DirectiveEndrepeat,
      "endscope" => TokenType::DirectiveEndscope,
      "endstruct" => TokenType::DirectiveEndstruct,
      "endunion" => TokenType::DirectiveEndunion,
      "enum" => TokenType::DirectiveEnum,
      "error" => TokenType::DirectiveError,
      "exitmac" => TokenType::DirectiveExitmac,
      "exitmacro" => TokenType::DirectiveExitmacro,
      "export" => TokenType::DirectiveExport,
      "exportzp" => TokenType::DirectiveExportzp,
      "faraddr" => TokenType::DirectiveFaraddr,
      "fatal" => TokenType::DirectiveFatal,
      "feature" => TokenType::DirectiveFeature,
      "fileopt" => TokenType::DirectiveFileopt,
      "fopt" => TokenType::DirectiveFopt,
      "forceimport" => TokenType::DirectiveForceimport,
      "forceword" => TokenType::DirectiveForceword,
      "global" => TokenType::DirectiveGlobal,
      "globalzp" => TokenType::DirectiveGlobalzp,
      "hibyte" => TokenType::DirectiveHibyte,
      "hibytes" => TokenType::DirectiveHibytes,
      "hiword" => TokenType::DirectiveHiword,
      "i16" => TokenType::DirectiveI16,
      "i8" => TokenType::DirectiveI8,
      "ident" => TokenType::DirectiveIdent,
      "if" => TokenType::DirectiveIf,
      "ifblank" => TokenType::DirectiveIfblank,
      "ifconst" => TokenType::DirectiveIfconst,
      "ifdef" => TokenType::DirectiveIfdef,
      "ifnblank" => TokenType::DirectiveIfnblank,
      "ifnconst" => TokenType::DirectiveIfnconst,
      "ifndef" => TokenType::DirectiveIfndef,
      "ifnref" => TokenType::DirectiveIfnref,
      "ifp02" => TokenType::DirectiveIfp02,
      "ifp4510" => TokenType::DirectiveIfp4510,
      "ifp816" => TokenType::DirectiveIfp816,
      "ifpc02" => TokenType::DirectiveIfpc02,
      "ifpsc02" => TokenType::DirectiveIfpsc02,
      "ifref" => TokenType::DirectiveIfref,
      "import" => TokenType::DirectiveImport,
      "importzp" => TokenType::DirectiveImportzp,
      "incbin" => TokenType::DirectiveIncbin,
      "include" => TokenType::DirectiveInclude,
      "interruptor" => TokenType::DirectiveInterruptor,
      "isize" => TokenType::DirectiveIsize,
      "ismnem" => TokenType::DirectiveIsmnem,
      "ismnemonic" => TokenType::DirectiveIsmnemonic,
      "left" => TokenType::DirectiveLeft,
      "linecont" => TokenType::DirectiveLinecont,
      "list" => TokenType::DirectiveList,
      "listbytes" => TokenType::DirectiveListbytes,
      "lobyte" => TokenType::DirectiveLobyte,
      "lobytes" => TokenType::DirectiveLobytes,
      "local" => TokenType::DirectiveLocal,
      "localchar" => TokenType::DirectiveLocalchar,
      "loword" => TokenType::DirectiveLoword,
      "mac" => TokenType::DirectiveMac,
      "macpack" => TokenType::DirectiveMacpack,
      "macro" => TokenType::DirectiveMacro,
      "match" => TokenType::DirectiveMatch,
      "max" => TokenType::DirectiveMax,
      "mid" => TokenType::DirectiveMid,
      "min" => TokenType::DirectiveMin,
      "mod" => TokenType::DirectiveMod,
      "not" => TokenType::DirectiveNot,
      "null" => TokenType::DirectiveNull,
      "or" => TokenType::DirectiveOr,
      "org" => TokenType::DirectiveOrg,
      "out" => TokenType::DirectiveOut,
      "p02" => TokenType::DirectiveP02,
      "p4510" => TokenType::DirectiveP4510,
      "p816" => TokenType::DirectiveP816,
      "pagelen" => TokenType::DirectivePagelen,
      "pagelength" => TokenType::DirectivePagelength,
      "paramcount" => TokenType::DirectiveParamcount,
      "pc02" => TokenType::DirectivePc02,
      "popcpu" => TokenType::DirectivePopcpu,
      "popseg" => TokenType::DirectivePopseg,
      "prog" => TokenType::DirectiveProc,
      "psc02" => TokenType::DirectivePsc02,
      "pushcpu" => TokenType::DirectivePushcpu,
      "pushseg" => TokenType::DirectivePushseg,
      "ref" => TokenType::DirectiveRef,
      "referenced" => TokenType::DirectiveReferenced,
      "reloc" => TokenType::DirectiveReloc,
      "repeat" => TokenType::DirectiveRepeat,
      "res" => TokenType::DirectiveRes,
      "right" => TokenType::DirectiveRight,
      "rodata" => TokenType::DirectiveRodata,
      "scope" => TokenType::DirectiveScope,
      "segment" => TokenType::DirectiveSegment,
      "set" => TokenType::DirectiveSet,
      "setcpu" => TokenType::DirectiveSetcpu,
      "shl" => TokenType::DirectiveShl,
      "shr" => TokenType::DirectiveShr,
      "sizeof" => TokenType::DirectiveSizeof,
      "smart" => TokenType::DirectiveSmart,
      "sprintf" => TokenType::DirectiveSprintf,
      "strat" => TokenType::DirectiveStrat,
      "string" => TokenType::DirectiveString,
      "strlen" => TokenType::DirectiveStrlen,
      "struct" => TokenType::DirectiveStruct,
      "tag" => TokenType::DirectiveTag,
      "tcount" => TokenType::DirectiveTcount,
      "time" => TokenType::DirectiveTime,
      "undef" => TokenType::DirectiveUndef,
      "undefine" => TokenType::DirectiveUndefine,
      "union" => TokenType::DirectiveUnion,
      "version" => TokenType::DirectiveVersion,
      "warning" => TokenType::DirectiveWarning,
      "word" => TokenType::DirectiveWord,
      "xmatch" => TokenType::DirectiveXmatch,
      "xor" => TokenType::DirectiveXor,
      "zeropage" => TokenType::DirectiveZeropage,
      _ => panic!("{} is not a directive", identifier),
    }
  }

  pub fn is_directive(&self) -> bool {
    match self {
      TokenType::DirectiveA16
      | TokenType::DirectiveA8
      | TokenType::DirectiveAddr
      | TokenType::DirectiveAddrsize
      | TokenType::DirectiveAlign
      | TokenType::DirectiveAnd
      | TokenType::DirectiveAsciiz
      | TokenType::DirectiveAsize
      | TokenType::DirectiveAssert
      | TokenType::DirectiveAutoimport
      | TokenType::DirectiveBank
      | TokenType::DirectiveBankbyte
      | TokenType::DirectiveBankbytes
      | TokenType::DirectiveBitand
      | TokenType::DirectiveBitnot
      | TokenType::DirectiveBitor
      | TokenType::DirectiveBitxor
      | TokenType::DirectiveBlank
      | TokenType::DirectiveBss
      | TokenType::DirectiveByt
      | TokenType::DirectiveByte
      | TokenType::DirectiveCase
      | TokenType::DirectiveCharmap
      | TokenType::DirectiveCode
      | TokenType::DirectiveConcat
      | TokenType::DirectiveCondes
      | TokenType::DirectiveConst
      | TokenType::DirectiveConstructor
      | TokenType::DirectiveCpu
      | TokenType::DirectiveData
      | TokenType::DirectiveDbg
      | TokenType::DirectiveDbyt
      | TokenType::DirectiveDebuginfo
      | TokenType::DirectiveDef
      | TokenType::DirectiveDefine
      | TokenType::DirectiveDefined
      | TokenType::DirectiveDefinedmacro
      | TokenType::DirectiveDelmac
      | TokenType::DirectiveDelmacro
      | TokenType::DirectiveDestructor
      | TokenType::DirectiveDword
      | TokenType::DirectiveElse
      | TokenType::DirectiveElseif
      | TokenType::DirectiveEnd
      | TokenType::DirectiveEndenum
      | TokenType::DirectiveEndif
      | TokenType::DirectiveEndmac
      | TokenType::DirectiveEndmacro
      | TokenType::DirectiveEndproc
      | TokenType::DirectiveEndrep
      | TokenType::DirectiveEndrepeat
      | TokenType::DirectiveEndscope
      | TokenType::DirectiveEndstruct
      | TokenType::DirectiveEndunion
      | TokenType::DirectiveEnum
      | TokenType::DirectiveError
      | TokenType::DirectiveExitmac
      | TokenType::DirectiveExitmacro
      | TokenType::DirectiveExport
      | TokenType::DirectiveExportzp
      | TokenType::DirectiveFaraddr
      | TokenType::DirectiveFatal
      | TokenType::DirectiveFeature
      | TokenType::DirectiveFileopt
      | TokenType::DirectiveFopt
      | TokenType::DirectiveForceimport
      | TokenType::DirectiveForceword
      | TokenType::DirectiveGlobal
      | TokenType::DirectiveGlobalzp
      | TokenType::DirectiveHibyte
      | TokenType::DirectiveHibytes
      | TokenType::DirectiveHiword
      | TokenType::DirectiveI16
      | TokenType::DirectiveI8
      | TokenType::DirectiveIdent
      | TokenType::DirectiveIf
      | TokenType::DirectiveIfblank
      | TokenType::DirectiveIfconst
      | TokenType::DirectiveIfdef
      | TokenType::DirectiveIfnblank
      | TokenType::DirectiveIfnconst
      | TokenType::DirectiveIfndef
      | TokenType::DirectiveIfnref
      | TokenType::DirectiveIfp02
      | TokenType::DirectiveIfp4510
      | TokenType::DirectiveIfp816
      | TokenType::DirectiveIfpc02
      | TokenType::DirectiveIfpsc02
      | TokenType::DirectiveIfref
      | TokenType::DirectiveImport
      | TokenType::DirectiveImportzp
      | TokenType::DirectiveIncbin
      | TokenType::DirectiveInclude
      | TokenType::DirectiveInterruptor
      | TokenType::DirectiveIsize
      | TokenType::DirectiveIsmnem
      | TokenType::DirectiveIsmnemonic
      | TokenType::DirectiveLeft
      | TokenType::DirectiveLinecont
      | TokenType::DirectiveList
      | TokenType::DirectiveListbytes
      | TokenType::DirectiveLobyte
      | TokenType::DirectiveLobytes
      | TokenType::DirectiveLocal
      | TokenType::DirectiveLocalchar
      | TokenType::DirectiveLoword
      | TokenType::DirectiveMac
      | TokenType::DirectiveMacpack
      | TokenType::DirectiveMacro
      | TokenType::DirectiveMatch
      | TokenType::DirectiveMax
      | TokenType::DirectiveMid
      | TokenType::DirectiveMin
      | TokenType::DirectiveMod
      | TokenType::DirectiveNot
      | TokenType::DirectiveNull
      | TokenType::DirectiveOr
      | TokenType::DirectiveOrg
      | TokenType::DirectiveOut
      | TokenType::DirectiveP02
      | TokenType::DirectiveP4510
      | TokenType::DirectiveP816
      | TokenType::DirectivePagelen
      | TokenType::DirectivePagelength
      | TokenType::DirectiveParamcount
      | TokenType::DirectivePc02
      | TokenType::DirectivePopcpu
      | TokenType::DirectivePopseg
      | TokenType::DirectiveProc
      | TokenType::DirectivePsc02
      | TokenType::DirectivePushcpu
      | TokenType::DirectivePushseg
      | TokenType::DirectiveRef
      | TokenType::DirectiveReferenced
      | TokenType::DirectiveReloc
      | TokenType::DirectiveRepeat
      | TokenType::DirectiveRes
      | TokenType::DirectiveRight
      | TokenType::DirectiveRodata
      | TokenType::DirectiveScope
      | TokenType::DirectiveSegment
      | TokenType::DirectiveSet
      | TokenType::DirectiveSetcpu
      | TokenType::DirectiveShl
      | TokenType::DirectiveShr
      | TokenType::DirectiveSizeof
      | TokenType::DirectiveSmart
      | TokenType::DirectiveSprintf
      | TokenType::DirectiveStrat
      | TokenType::DirectiveString
      | TokenType::DirectiveStrlen
      | TokenType::DirectiveStruct
      | TokenType::DirectiveTag
      | TokenType::DirectiveTcount
      | TokenType::DirectiveTime
      | TokenType::DirectiveUndef
      | TokenType::DirectiveUndefine
      | TokenType::DirectiveUnion
      | TokenType::DirectiveVersion
      | TokenType::DirectiveWarning
      | TokenType::DirectiveWord
      | TokenType::DirectiveXmatch
      | TokenType::DirectiveXor
      | TokenType::DirectiveZeropage => true,
      _ => false,
    }
  }
}
