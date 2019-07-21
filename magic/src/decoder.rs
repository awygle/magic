use magic_macros::vr4300_instr_enum;

vr4300_instr_enum!(CpuInstrVR4300);
use CpuInstrVR4300::*;

const DECODER_LOOKUP_TABLE :[CpuInstrVR4300; 312] = [
// ============================================================================
//  Escaped opcode table: Special.
//
//      31---------26------------------------------------------5--------0
//      | SPECIAL/6 |                                         |  FMT/6  |
//      ------6----------------------------------------------------6-----
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//  000 | SLL   |       | SRL   | SRA   | SLLV  |       | SRLV  | SRAV  |
//  001 | JR    | JALR  |       |       |SYSCALL| BREAK |       | SYNC  |
//  010 | MFHI  | MTHI  | MFLO  | MTLO  | DSLLV |       | DSRLV | DSRAV |
//  011 | MULT  | MULTU | DIV   | DIVU  | DMULT | DMULTU| DDIV  | DDIVU |
//  100 | ADD   | ADDU  | SUB   | SUBU  | AND   | OR    | XOR   | NOR   |
//  101 |       |       | SLT   | SLTU  | DADD  | DADDU | DSUB  | DSUBU |
//  110 | TGE   | TGEU  | TLT   | TLTU  | TEQ   |       | TNE   |       |
//  111 | DSLL  |       | DSRL  | DSRA  |DSLL32 |       |DSRL32 |DSRA32 |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
//
// ============================================================================
    {SLL},     {Invalid}, {SRL},     {SRA},
    {SLLV},    {Invalid}, {SRLV},    {SRAV},
    {JR},      {JALR},    {Invalid}, {Invalid},
    {SYSCALL}, {BREAK},   {Invalid}, {SYNC},
    {MFHI},    {MTHI},    {MFLO},    {MTLO},
    {DSLLV},   {Invalid}, {DSRLV},   {DSRAV},
    {MULT},    {MULTU},   {DIV},     {DIVU},
    {DMULT},   {DMULTU},  {DDIV},    {DDIVU},
    {ADD},     {ADDU},    {SUB},     {SUBU},
    {AND},     {OR},      {XOR},     {NOR},
    {Invalid}, {Invalid}, {SLT},     {SLTU},
    {DADD},    {DADDU},   {DSUB},    {DSUBU},
    {TGE},     {TGEU},    {TLT},     {TLTU},
    {TEQ},     {Invalid}, {TNE},     {Invalid},
    {DSLL},    {Invalid}, {DSRL},    {DSRA},
    {DSLL32},  {Invalid}, {DSRL32},  {DSRA32},
    
// ============================================================================
//  Escaped opcode table: RegImm.
//
//      31---------26----------20-------16------------------------------0
//      | = REGIMM  |          |  FMT/5  |                              |
//      ------6---------------------5------------------------------------
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//   00 | BLTZ  | BGEZ  | BLTZL | BGEZL |       |       |       |       |
//   01 | TGEI  | TGEIU | TLTI  | TLTIU | TEQI  |       | TNEI  |       |
//   10 | BLTZAL| BGEZAL|BLTZALL|BGEZALL|       |       |       |       |
//   11 |       |       |       |       |       |       |       |       |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
//
// ============================================================================
    {BLTZ},    {BGEZ},    {BLTZL},   {BGEZL},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {TGEI},    {TGEIU},   {TLTI},    {TLTIU},
    {TEQI},    {Invalid}, {TNEI},    {Invalid},
    {BLTZAL},  {BGEZAL},  {BLTZALL}, {BGEZALL},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},

// ============================================================================
//  Escaped opcode table: COP0/0.
//
//      31--------26-25--24-----21--------------------------------------0
//      |   COP0/6  | 0 |  FMT/4  |                                     |
//      ------6-------1------4------------------------------------------0
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//   00 | MFC0  | DMFC0 |  ---  |  ---  | MTC0  | DMTC0 |  ---  |  ---  |
//   01 |  BC0  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   10 |  TLB  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   11 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ============================================================================
    {MFC0},    {DMFC0},   {Invalid}, {Invalid},
    {MTC0},    {DMTC0},   {Invalid}, {Invalid},

// ============================================================================
//  Escaped opcode table: COP0/2.
//
//      31--------26-25--23-----------------------------------5---------0
//      |   COP0/6  | 10 |                                    |  FMT/6  |
//      ------6-------2--------------------------------------------6-----
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//  000 |  ---  | TLBR  | TLBWI |  ---  |  ---  |  ---  | TLBWR |  ---  |
//  001 | TLBP  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  010 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  011 | ERET  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  100 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  101 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  110 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  111 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ========================================================================= */
    {Invalid}, {TLBR},    {TLBWI},   {Invalid},
    {Invalid}, {Invalid}, {TLBWR},   {Invalid},
    {TLBP},    {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {ERET},    {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},
    {Invalid}, {Invalid}, {Invalid}, {Invalid},

// ============================================================================
//  Escaped opcode table: COP0/3.
//
//      31--------26-25--24------21------16-----------------------------0
//      |   COP0/6  | 11 |        | FMT/5 |                             |
//      ------6-------2-------3-------5---------------------------------0
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//   00 |  BCF  |  BCT  |  BCFL |  BCTL |  ---  |  --- |  ---  |  ---  |
//   01 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   10 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   11 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ============================================================================
    {BC0F},    {BC0T},   {BC0FL}, {BC0TL},

// ============================================================================
//  Escaped opcode table: COP1/1.
//
//      31--------26-25------21 ----------------------------------------0
//      |  COP1/6   |  FMT/5  |                                         |
//      ------6----------5-----------------------------------------------
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//   00 | MFC1  | DMFC1 | CFC1  |  ---  | MTC1  | DMTC1 | CTC1  |  ---  |
//   01 |  BC1  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   10 | FPUS  | FPUD  |  ---  |  ---  | FPUW  | FPUL  |  ---  |  ---  |
//   11 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ============================================================================
    {MFC1},    {DMFC1},   {CFC1},    {Invalid},
    {MTC1},    {DMTC1},   {CTC1},    {Invalid},
    
// ============================================================================
//  Escaped opcode table: COP1/2.
//
//      31--------26-25 -24-----------------------------------5---------0
//      |   COP1/6  | 1 |                                     |  FMT/6  |
//      ------6-------1--------------------------------------------6-----
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//  000 |  ADD  |  SUB  |  MUL  |  DIV  | SQRT  |  ABS  |  MOV  |  NEG  |
//  001 |ROUND.L|TRUNC.L|CEIL.L |FLOOR.L|ROUND.W|TRUNC.W|CEIL.W |FLOOR.W|
//  010 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  011 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  100 | CVT.S | CVT.D |  ---  |  ---  | CVT.W | CVT.L |  ---  |  ---  |
//  101 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//  110 |  C.F  | C.UN  | C.EQ  | C.UEQ | C.OLT | C.ULT | C.OLE | C.ULE |
//  111 | C.SF  |C.NGLE | C.SEQ | C.NGL | C.LT  | C.NGE | C.LE  | C.NGT |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ========================================================================= */
    {ADD_S},     {SUB_S},     {MUL_S},     {DIV_S}, // All of these are set to _S to get it to build, fixed later(?)
    {SQRT_S},    {ABS_S},     {MOV_S},     {NEG_S},
    {ROUND_L_S}, {TRUNC_L_S}, {CEIL_L_S},  {FLOOR_L_S},
    {ROUND_W_S}, {TRUNC_W_S}, {CEIL_W_S},  {FLOOR_W_S},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {CVT_S_D},   {CVT_D_S},   {Invalid},     {Invalid},
    {CVT_W_S},   {CVT_L_S},   {Invalid},     {Invalid},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {Invalid},     {Invalid},     {Invalid},     {Invalid},
    {C_F_S},     {C_UN_S},    {C_EQ_S},    {C_UEQ_S},
    {C_OLT_S},   {C_ULT_S},   {C_OLE_S},   {C_ULE_S},
    {C_SF_S},    {C_NGLE_S},  {C_SEQ_S},   {C_NGL_S},
    {C_LT_S},    {C_NGE_S},   {C_LE_S},    {C_NGT_S},
    
// ============================================================================
//  Escaped opcode table: COP1/3.
//
//      31--------26-25--24------21------16-----------------------------0
//      |   COP1/6  | 11 |        | FMT/5 |                             |
//      ------6-------2-------3-------5---------------------------------0
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//   00 |  BCF  |  BCT  |  BCFL |  BCTL |  ---  |  --- |  ---  |  ---  |
//   01 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   10 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//   11 |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |  ---  |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
// ============================================================================
    {BC1F},    {BC1T},   {BC1FL}, {BC1TL},

// ============================================================================
//  First-order opcode table.
//
//      31---------26---------------------------------------------------0
//      |  OPCODE/6 |                                                   |
//      ------6----------------------------------------------------------
//      |--000--|--001--|--010--|--011--|--100--|--101--|--110--|--111--|
//  000 | *SPEC | *RGIM | J     | JAL   | BEQ   | BNE   | BLEZ  | BGTZ  |
//  001 | ADDI  | ADDIU | SLTI  | SLTIU | ANDI  | ORI   | XORI  | LUI   |
//  010 | *COP0 | *COP1 |       |       | BEQL  | BNEL  | BLEZL | BGTZL |
//  011 | DADDI |DADDIU |  LDL  |  LDR  |       |       |       |       |
//  100 | LB    | LH    | LWL   | LW    | LBU   | LHU   | LWR   | LWU   |
//  101 | SB    | SH    | SWL   | SW    | SDL   | SDR   | SWR   | CACHE |
//  110 | LL    | LWC1  |       |       | LLD   | LDC1  |       | LD    |
//  111 | SC    | SWC1  |       |       | SCD   | SDC1  |       | SD    |
//      |-------|-------|-------|-------|-------|-------|-------|-------|
//
// ============================================================================
  {Invalid}, {Invalid}, {J},       {JAL},
  {BEQ},     {BNE},     {BLEZ},    {BGTZ},
  {ADDI},    {ADDIU},   {SLTI},    {SLTIU},
  {ANDI},    {ORI},     {XORI},    {LUI},
  {Invalid}, {Invalid}, {Invalid}, {Invalid},
  {BEQL},    {BNEL},    {BLEZL},   {BGTZL},
  {DADDI},   {DADDIU},  {LDL},     {LDR},
  {Invalid}, {Invalid}, {Invalid}, {Invalid},
  {LB},      {LH},      {LWL},     {LW},
  {LBU},     {LHU},     {LWR},     {LWU},
  {SB},      {SH},      {SWL},     {SW},
  {SDL},     {SDR},     {SWR},     {CACHE},
  {LL},      {LWC1},    {Invalid},    {Invalid},
  {LLD},     {LDC1},    {Invalid},    {LD},
  {SC},      {SWC1},    {Invalid},    {Invalid},
  {SCD},     {SDC1},    {Invalid},    {SD}
];

#[derive(Debug, Copy, Clone)]
struct OpcodeEscape {
    shift: u8,
    mask: u8,
    offset: u16
}

const SPEC_OFFSET: u16 = 0;
const FUNC_MASK: u8 = 0x3F;
const FUNC_SHIFT: u8 = 0;
const REGIMM_OFFSET: u16 = 64;
const RT_MASK: u8 = 0x1F;
const RT_SHIFT: u8 = 16;
const DEFAULT_OFFSET: u16 = 248;
const OPCODE_MASK: u8 = 0x3F;
const OPCODE_SHIFT: u8 = 26;
const COP0_0_OFFSET: u16 = 96;
const RS_MASK_SHORT: u8 = 0x7;
const RS_SHIFT: u8 = 21;
const COP0_2_OFFSET: u16 = 104;
const COP0_3_OFFSET: u16 = 168;
const RT_MASK_SHORT: u8 = 0x3;
const COP1_0_OFFSET: u16 = 172;
const COP1_2_OFFSET: u16 = 180;
const COP1_3_OFFSET: u16 = 244;
const INVALID_OFFSET: u16 = 65535;

const OPCODE_ESCAPE_TABLE: [OpcodeEscape; 256] = [
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: SPEC_OFFSET}, // Special
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: SPEC_OFFSET},
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: SPEC_OFFSET},
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: SPEC_OFFSET},
    
    OpcodeEscape {mask: RT_MASK, shift: RT_SHIFT, offset: REGIMM_OFFSET}, // RegImm
    OpcodeEscape {mask: RT_MASK, shift: RT_SHIFT, offset: REGIMM_OFFSET},
    OpcodeEscape {mask: RT_MASK, shift: RT_SHIFT, offset: REGIMM_OFFSET},
    OpcodeEscape {mask: RT_MASK, shift: RT_SHIFT, offset: REGIMM_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // J
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // JAL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BEQ
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BNE
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BLEZ
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BGTZ
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // ADDI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // ADDIU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SLTI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SLTIU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // ANDI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // ORI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // XORI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LUI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: RS_MASK_SHORT, shift: RS_SHIFT, offset: COP0_0_OFFSET}, // COP0_0
    OpcodeEscape {mask: RS_MASK_SHORT, shift: RS_SHIFT, offset: COP0_0_OFFSET}, // COP0_0
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: COP0_2_OFFSET}, // COP0_2
    OpcodeEscape {mask: RT_MASK_SHORT, shift: RT_SHIFT, offset: COP0_3_OFFSET}, // COP0_3
    
    OpcodeEscape {mask: RS_MASK_SHORT, shift: RS_SHIFT, offset: COP1_0_OFFSET}, // COP1_0
    OpcodeEscape {mask: RS_MASK_SHORT, shift: RS_SHIFT, offset: COP1_0_OFFSET}, // COP1_0
    OpcodeEscape {mask: FUNC_MASK, shift: FUNC_SHIFT, offset: COP1_2_OFFSET}, // COP1_2
    OpcodeEscape {mask: RT_MASK_SHORT, shift: RT_SHIFT, offset: COP1_3_OFFSET}, // COP1_3
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BEQL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BNEL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BLEZL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // BGTZL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // DADDI
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // DADDIU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LDL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LDR
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LB
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LH
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LWL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LW
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LBU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LHU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LWR
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LWU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SB
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SH
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SWL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SW
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SBU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SHU
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SWR
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // CACHE
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LL
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LWC1
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LLD
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LDC1
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // LD
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SC
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SWC1
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SCD
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SDC1
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET}, // INVALID
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: INVALID_OFFSET},
    
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET}, // SD
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    OpcodeEscape {mask: OPCODE_MASK, shift: OPCODE_SHIFT, offset: DEFAULT_OFFSET},
    
];

//const struct vr4300_opcode* vr4300_decode_instruction(uint32_t iw) {
//  const struct vr4300_opcode_escape *escape = vr4300_escape_table + (iw >> 25);
//  unsigned index = iw >> escape->shift & escape->mask;
//
//  const struct vr4300_opcode *group = vr4300_opcode_table + escape->offset;
//  return group + index;
//}

// Scalar decoder
fn decode_vr4300(iw: u32) -> CpuInstrVR4300 {
    let escape = OPCODE_ESCAPE_TABLE[(iw as usize) >> 24];
    let index = (iw >> escape.shift) & (escape.mask as u32);
    
    DECODER_LOOKUP_TABLE[(escape.offset as usize) + (index as usize)]
}
