use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
  val: String,
  t_type: TokenType,
  start: usize,
  end: usize,
}

impl Token {
  pub fn new(val: String, t: TokenType, start: usize, end: usize) -> Token {
    Token {
      val: val,
      t_type: t,
      start: start,
      end: end,
    }
  }

  pub fn get_type(&self) -> &TokenType {
    &self.t_type
  }

  pub fn get_value(&self) -> &String {
    &self.val
  }

  pub fn get_start(&self) -> &usize {
    &self.start
  }

  pub fn get_end(&self) -> &usize {
    &self.end
  }
}

impl fmt::Display for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "type: {:?}, text: {}, start: {}, end: {}",
      self.t_type, self.val, self.start, self.end
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
  Number,
  HexNumber,
  BinNumber,
  DecNumber,
  A16,
  A8,
  Addr,
  Addrsize,
  Align,
  And,
  Asciiz,
  Asize,
  Assert,
  Autoimport,
  Bank,
  Bankbyte,
  Bankbytes,
  Bitand,
  Bitnot,
  Bitor,
  Bitxor,
  Blank,
  Bss,
  Byt,
  Byte,
  Case,
  Charmap,
  Code,
  Concat,
  Condes,
  Const,
  Constructor,
  Cpu,
  Data,
  Dbg,
  Dbyt,
  Debuginfo,
  Def,
  Define,
  Defined,
  Definedmacro,
  Delmac,
  Delmacro,
  Destructor,
  Dword,
  Else,
  Elseif,
  End,
  Endenum,
  Endif,
  Endmac,
  Endmacro,
  Endproc,
  Endrep,
  Endrepeat,
  Endscope,
  Endstruct,
  Endunion,
  Enum,
  Error,
  Exitmac,
  Exitmacro,
  Export,
  Exportzp,
  Faraddr,
  Fatal,
  Feature,
  Fileopt,
  Fopt,
  Forceimport,
  Forceword,
  Global,
  Globalzp,
  Hibyte,
  Hibytes,
  Hiword,
  I16,
  I8,
  Ident,
  If,
  Ifblank,
  Ifconst,
  Ifdef,
  Ifnblank,
  Ifnconst,
  Ifndef,
  Ifnref,
  Ifp02,
  Ifp4510,
  Ifp816,
  Ifpc02,
  Ifpsc02,
  Ifref,
  Import,
  Importzp,
  Incbin,
  Include,
  Interruptor,
  Isize,
  Ismnem,
  Ismnemonic,
  Left,
  Linecont,
  List,
  Listbytes,
  Lobyte,
  Lobytes,
  Local,
  Localchar,
  Loword,
  Mac,
  Macpack,
  Macro,
  Match,
  Max,
  Mid,
  Min,
  Mod,
  Not,
  Null,
  Or,
  Org,
  Out,
  P02,
  P4510,
  P816,
  Pagelen,
  Pagelength,
  Paramcount,
  Pc02,
  Popcpu,
  Popseg,
  Proc,
  Psc02,
  Pushcpu,
  Pushseg,
  Ref,
  Referenced,
  Reloc,
  Repeat,
  Res,
  Right,
  Rodata,
  Scope,
  Segment,
  Set,
  Setcpu,
  Shl,
  Shr,
  Sizeof,
  Smart,
  Sprintf,
  Strat,
  String,
  Strlen,
  Struct,
  Tag,
  Tcount,
  Time,
  Undef,
  Undefine,
  Union,
  Version,
  Warning,
  Word,
  Xmatch,
  Xor,
  Zeropage,
  LocalLabel,
  XRegister,
  YRegister,
  EndOfFile,
}

impl TokenType {
  pub fn get_directive_type(identifier: &str) -> TokenType {
    match identifier {
      "a16" => TokenType::A16,
      "a8" => TokenType::A8,
      "addr" => TokenType::Addr,
      "addrsize" => TokenType::Addrsize,
      "align" => TokenType::Align,
      "and" => TokenType::And,
      "asciiz" => TokenType::Asciiz,
      "asize" => TokenType::Asize,
      "assert" => TokenType::Assert,
      "autoimport" => TokenType::Autoimport,
      "bank" => TokenType::Bank,
      "bankbyte" => TokenType::Bankbyte,
      "bankbytes" => TokenType::Bankbytes,
      "bitand" => TokenType::Bitand,
      "bitnot" => TokenType::Bitnot,
      "bitor" => TokenType::Bitor,
      "bitxor" => TokenType::Bitxor,
      "blank" => TokenType::Blank,
      "bss" => TokenType::Bss,
      "byt" => TokenType::Byt,
      "byte" => TokenType::Byte,
      "case" => TokenType::Case,
      "charmap" => TokenType::Charmap,
      "code" => TokenType::Code,
      "concat" => TokenType::Concat,
      "condes" => TokenType::Condes,
      "const" => TokenType::Const,
      "constructor" => TokenType::Constructor,
      "cpu" => TokenType::Cpu,
      "data" => TokenType::Data,
      "dbg" => TokenType::Dbg,
      "dbyt" => TokenType::Dbyt,
      "debuginfo" => TokenType::Debuginfo,
      "def" => TokenType::Def,
      "define" => TokenType::Define,
      "defined" => TokenType::Defined,
      "definedmacro" => TokenType::Definedmacro,
      "delmac" => TokenType::Delmac,
      "delmacro" => TokenType::Delmacro,
      "destructor" => TokenType::Destructor,
      "dword" => TokenType::Dword,
      "else" => TokenType::Else,
      "elseif" => TokenType::Elseif,
      "end" => TokenType::End,
      "endenum" => TokenType::Endenum,
      "endif" => TokenType::Endif,
      "endmac" => TokenType::Endmac,
      "endmacro" => TokenType::Endmacro,
      "endproc" => TokenType::Endproc,
      "endrep" => TokenType::Endrep,
      "endrepeat" => TokenType::Endrepeat,
      "endscope" => TokenType::Endscope,
      "endstruct" => TokenType::Endstruct,
      "endunion" => TokenType::Endunion,
      "enum" => TokenType::Enum,
      "error" => TokenType::Error,
      "exitmac" => TokenType::Exitmac,
      "exitmacro" => TokenType::Exitmacro,
      "export" => TokenType::Export,
      "exportzp" => TokenType::Exportzp,
      "faraddr" => TokenType::Faraddr,
      "fatal" => TokenType::Fatal,
      "feature" => TokenType::Feature,
      "fileopt" => TokenType::Fileopt,
      "fopt" => TokenType::Fopt,
      "forceimport" => TokenType::Forceimport,
      "forceword" => TokenType::Forceword,
      "global" => TokenType::Global,
      "globalzp" => TokenType::Globalzp,
      "hibyte" => TokenType::Hibyte,
      "hibytes" => TokenType::Hibytes,
      "hiword" => TokenType::Hiword,
      "i16" => TokenType::I16,
      "i8" => TokenType::I8,
      "ident" => TokenType::Ident,
      "if" => TokenType::If,
      "ifblank" => TokenType::Ifblank,
      "ifconst" => TokenType::Ifconst,
      "ifdef" => TokenType::Ifdef,
      "ifnblank" => TokenType::Ifnblank,
      "ifnconst" => TokenType::Ifnconst,
      "ifndef" => TokenType::Ifndef,
      "ifnref" => TokenType::Ifnref,
      "ifp02" => TokenType::Ifp02,
      "ifp4510" => TokenType::Ifp4510,
      "ifp816" => TokenType::Ifp816,
      "ifpc02" => TokenType::Ifpc02,
      "ifpsc02" => TokenType::Ifpsc02,
      "ifref" => TokenType::Ifref,
      "import" => TokenType::Import,
      "importzp" => TokenType::Importzp,
      "incbin" => TokenType::Incbin,
      "include" => TokenType::Include,
      "interruptor" => TokenType::Interruptor,
      "isize" => TokenType::Isize,
      "ismnem" => TokenType::Ismnem,
      "ismnemonic" => TokenType::Ismnemonic,
      "left" => TokenType::Left,
      "linecont" => TokenType::Linecont,
      "list" => TokenType::List,
      "listbytes" => TokenType::Listbytes,
      "lobyte" => TokenType::Lobyte,
      "lobytes" => TokenType::Lobytes,
      "local" => TokenType::Local,
      "localchar" => TokenType::Localchar,
      "loword" => TokenType::Loword,
      "mac" => TokenType::Mac,
      "macpack" => TokenType::Macpack,
      "macro" => TokenType::Macro,
      "match" => TokenType::Match,
      "max" => TokenType::Max,
      "mid" => TokenType::Mid,
      "min" => TokenType::Min,
      "mod" => TokenType::Mod,
      "not" => TokenType::Not,
      "null" => TokenType::Null,
      "or" => TokenType::Or,
      "org" => TokenType::Org,
      "out" => TokenType::Out,
      "p02" => TokenType::P02,
      "p4510" => TokenType::P4510,
      "p816" => TokenType::P816,
      "pagelen" => TokenType::Pagelen,
      "pagelength" => TokenType::Pagelength,
      "paramcount" => TokenType::Paramcount,
      "pc02" => TokenType::Pc02,
      "popcpu" => TokenType::Popcpu,
      "popseg" => TokenType::Popseg,
      "prog" => TokenType::Proc,
      "psc02" => TokenType::Psc02,
      "pushcpu" => TokenType::Pushcpu,
      "pushseg" => TokenType::Pushseg,
      "ref" => TokenType::Ref,
      "referenced" => TokenType::Referenced,
      "reloc" => TokenType::Reloc,
      "repeat" => TokenType::Repeat,
      "res" => TokenType::Res,
      "right" => TokenType::Right,
      "rodata" => TokenType::Rodata,
      "scope" => TokenType::Scope,
      "segment" => TokenType::Segment,
      "set" => TokenType::Set,
      "setcpu" => TokenType::Setcpu,
      "shl" => TokenType::Shl,
      "shr" => TokenType::Shr,
      "sizeof" => TokenType::Sizeof,
      "smart" => TokenType::Smart,
      "sprintf" => TokenType::Sprintf,
      "strat" => TokenType::Strat,
      "string" => TokenType::String,
      "strlen" => TokenType::Strlen,
      "struct" => TokenType::Struct,
      "tag" => TokenType::Tag,
      "tcount" => TokenType::Tcount,
      "time" => TokenType::Time,
      "undef" => TokenType::Undef,
      "undefine" => TokenType::Undefine,
      "union" => TokenType::Union,
      "version" => TokenType::Version,
      "warning" => TokenType::Warning,
      "word" => TokenType::Word,
      "xmatch" => TokenType::Xmatch,
      "xor" => TokenType::Xor,
      "zeropage" => TokenType::Zeropage,
      _ => panic!("{} is not a directive"),
    }
  }
}
