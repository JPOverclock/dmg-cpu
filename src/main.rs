const OP_CODES: [OpCode; 0x100] = [
    // 0x
    OpCode{ mnemonic: "NOP", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD BC,d16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "LD (BC),A", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC BC", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLCA", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD (a16),SP", length: 3, cycles: 20 },
    OpCode{ mnemonic: "ADD HL,BC", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,(BC)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "DEC BC", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRCA", length: 1, cycles: 4 },
    // 1x
    OpCode{ mnemonic: "STOP 0", length: 2, cycles: 4 },
    OpCode{ mnemonic: "LD DE,d16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "LD (DE),A", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC DE", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLA", length: 1, cycles: 4 },
    OpCode{ mnemonic: "JR r8", length: 2, cycles: 12 },
    OpCode{ mnemonic: "ADD HL,DE", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,(DE)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "DEC DE", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRA", length: 1, cycles: 4 },
    // 2x
    OpCode{ mnemonic: "JR NZ,r8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "LD HL,d16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "LD (HL+),A", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC HL", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "DAA", length: 1, cycles: 4 },
    OpCode{ mnemonic: "JR Z,r8", length: 3, cycles: 8 },
    OpCode{ mnemonic: "ADD HL,HL", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,(HL+)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "DEC HL", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "CPL", length: 1, cycles: 4 },
    // 3x
    OpCode{ mnemonic: "JR NC,r8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "LD SP,d16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "LD (HL-),A", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC SP", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC (HL)", length: 1, cycles: 12 },
    OpCode{ mnemonic: "DEC (HL)", length: 1, cycles: 12 },
    OpCode{ mnemonic: "LD (HL),d8", length: 2, cycles: 12 },
    OpCode{ mnemonic: "SCF", length: 1, cycles: 4 },
    OpCode{ mnemonic: "JR C,r8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "ADD HL,SP", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,(HL-)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "DEC SP", length: 1, cycles: 8 },
    OpCode{ mnemonic: "INC A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "DEC A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "CCF", length: 1, cycles: 4 },
    // 4x
    OpCode{ mnemonic: "LD B,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD B,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD B,A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD C,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD C,A", length: 1, cycles: 4 },
    // 5x
    OpCode{ mnemonic: "LD D,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD D,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD D,A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD E,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD E,A", length: 1, cycles: 4 },
    // 6x
    OpCode{ mnemonic: "LD H,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD H,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD H,A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD L,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD L,A", length: 1, cycles: 1 },
    // 7x
    OpCode{ mnemonic: "LD (HL),B", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD (HL),C", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD (HL),D", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD (HL),E", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD (HL),H", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD (HL),L", length: 1, cycles: 8 },
    OpCode{ mnemonic: "HALT", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD (HL),A", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD A,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,A", length: 1, cycles: 4 },
    // 8x
    OpCode{ mnemonic: "ADD A,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADD A,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "ADD A,A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "ADC A,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "ADC A,A", length: 1, cycles: 4 },
    // 9x
    OpCode{ mnemonic: "SUB B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SUB A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,(HL)", length: 1, cycles: 8 },
    OpCode{ mnemonic: "SBC A,A", length: 1, cycles: 1 },
    // Ax
    OpCode{ mnemonic: "AND B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "AND A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR A", length: 1, cycles: 4 },
    // Bx
    OpCode{ mnemonic: "OR B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "OR A", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP B", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP C", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP D", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP E", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP H", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP L", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP A", length: 1, cycles: 4 },
    // Cx
    OpCode{ mnemonic: "RET NZ", length: 1, cycles: 8 },
    OpCode{ mnemonic: "POP BC", length: 1, cycles: 12 },
    OpCode{ mnemonic: "JP NZ,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "JP a16", length: 3, cycles: 16 },
    OpCode{ mnemonic: "CALL NZ,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "PUSH BC", length: 1, cycles: 16 },
    OpCode{ mnemonic: "ADD A,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 00H", length: 1, cycles: 16 },
    OpCode{ mnemonic: "RET Z", length: 1, cycles: 8 },
    OpCode{ mnemonic: "RET", length: 1, cycles: 16 },
    OpCode{ mnemonic: "JP Z,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "PREFIX CB", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CALL Z,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "CALL a16", length: 3, cycles: 24 },
    OpCode{ mnemonic: "ADC A,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 08H", length: 1, cycles: 16 },
    // Dx
    OpCode{ mnemonic: "RET NC", length: 1, cycles: 8 },
    OpCode{ mnemonic: "POP DE", length: 1, cycles: 12 },
    OpCode{ mnemonic: "JP NC,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CALL NC,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "PUSH DE", length: 1, cycles: 16 },
    OpCode{ mnemonic: "SUB d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 10H", length: 1, cycles: 16 },
    OpCode{ mnemonic: "RET C", length: 1, cycles: 8 },
    OpCode{ mnemonic: "RETI", length: 1, cycles: 16 },
    OpCode{ mnemonic: "JP C,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CALL C,a16", length: 3, cycles: 12 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "SBC A,d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 18H", length: 1, cycles: 16 },
    // Ex
    OpCode{ mnemonic: "LDH (a8),A", length: 2, cycles: 12 },
    OpCode{ mnemonic: "POP HL", length: 1, cycles: 12 },
    OpCode{ mnemonic: "LD (C),A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "???", length: 1, cycles: 1 },
    OpCode{ mnemonic: "???", length: 1, cycles: 1 },
    OpCode{ mnemonic: "PUSH HL", length: 1, cycles: 16 },
    OpCode{ mnemonic: "AND d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 20H", length: 1, cycles: 16 },
    OpCode{ mnemonic: "ADD SP,r8", length: 2, cycles: 16 },
    OpCode{ mnemonic: "JP (HL)", length: 1, cycles: 4 },
    OpCode{ mnemonic: "LD (a16),A", length: 3, cycles: 16 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "XOR d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 28H", length: 1, cycles: 16 },
    // Fx
    OpCode{ mnemonic: "LDH A,(a8)", length: 2, cycles: 12 },
    OpCode{ mnemonic: "POP AF", length: 1, cycles: 12 },
    OpCode{ mnemonic: "LD A,(C)", length: 2, cycles: 8 },
    OpCode{ mnemonic: "DI", length: 1, cycles: 4 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "PUSH AF", length: 1, cycles: 16 },
    OpCode{ mnemonic: "OR d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 30H", length: 1, cycles: 16 },
    OpCode{ mnemonic: "LD HL,SP+r8", length: 2, cycles: 16 },
    OpCode{ mnemonic: "LD SP,HL", length: 1, cycles: 8 },
    OpCode{ mnemonic: "LD A,(a16)", length: 3, cycles: 16 },
    OpCode{ mnemonic: "EI", length: 1, cycles: 4 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "???", length: 1, cycles: 4 },
    OpCode{ mnemonic: "CP d8", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RST 38H", length: 1, cycles: 16 },
];

const CB_PREFIX_OP_CODES: [OpCode; 0x100] = [
    // 0x
    OpCode{ mnemonic: "RLC B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RLC (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RLC A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RRC (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RRC A", length: 2, cycles: 8 },
    // 1x
    OpCode{ mnemonic: "RL B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RL (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RL A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RR (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RR A", length: 2, cycles: 8 },
    // 2x
    OpCode{ mnemonic: "SLA B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SLA (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SLA A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRA (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SRA A", length: 2, cycles: 8 },
    // 3x
    OpCode{ mnemonic: "SWAP B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SWAP (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SWAP A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SRL (HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SRL A", length: 2, cycles: 8 },
    // 4x
    OpCode{ mnemonic: "BIT 0,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 0,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 0,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 1,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 1,A", length: 2, cycles: 8 },
    // 5x
    OpCode{ mnemonic: "BIT 2,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 2,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 2,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 3,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 3,A", length: 2, cycles: 8 },
    // 6x
    OpCode{ mnemonic: "BIT 4,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 4,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 4,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 5,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 5,A", length: 2, cycles: 8 },
    // 7x
    OpCode{ mnemonic: "BIT 6,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 6,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 6,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "BIT 7,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "BIT 7,A", length: 2, cycles: 8 },
    // 8x
    OpCode{ mnemonic: "RES 0,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 0,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 0,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 1,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 1,A", length: 2, cycles: 8 },
    // 9x
    OpCode{ mnemonic: "RES 2,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 2,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 2,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 3,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 3,A", length: 2, cycles: 8 },
    // Ax
    OpCode{ mnemonic: "RES 4,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 4,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 4,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 5,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 5,A", length: 2, cycles: 8 },
    // Bx
    OpCode{ mnemonic: "RES 6,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 6,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 6,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "RES 7,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "RES 7,A", length: 2, cycles: 8 },
    // Cx
    OpCode{ mnemonic: "SET 0,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 0,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 0,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 1,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 1,A", length: 2, cycles: 8 },
    // Dx
    OpCode{ mnemonic: "SET 2,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 2,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 2,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 3,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 3,A", length: 2, cycles: 8 },
    // Ex
    OpCode{ mnemonic: "SET 4,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 4,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 4,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 5,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 5,A", length: 2, cycles: 8 },
    // Fx
    OpCode{ mnemonic: "SET 6,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 6,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 6,A", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,B", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,C", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,D", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,E", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,H", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,L", length: 2, cycles: 8 },
    OpCode{ mnemonic: "SET 7,(HL)", length: 2, cycles: 16 },
    OpCode{ mnemonic: "SET 7,A", length: 2, cycles: 8 },
];

trait Bus {
    fn write(&mut self, address: u16, data: u8);
    fn read(&mut self, address: u16) -> u8;
}

struct OpCode {
    mnemonic: &'static str,
    length: u16,
    cycles: u8,
}

struct MemoryBus {
    memory: [u8; 0x10000]
}

impl Bus for MemoryBus {
    fn write(&mut self, address: u16, data: u8) {
        self.memory[usize::from(address)] = data
    }

    fn read(&mut self, address: u16) -> u8 {
        self.memory[usize::from(address)]
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

struct CPU<'bus, T: Bus> {
    registers: Registers,
    pc: u16,
    sp: u16,
    wait: u8,
    bus: &'bus mut T,
}

impl <'bus, T: Bus> CPU<'bus, T> {
    fn clock(&mut self) {
        if self.wait == 0 {
            // Read data from PC
            let data = self.bus.read(self.pc);

            // Decode instruction
            let op_code = &OP_CODES[usize::from(data)];

            println!("[${}] Fetched code {} [C: {}, L: {}] from position {}", self.pc, op_code.mnemonic, op_code.cycles, op_code.length, usize::from(data));

            self.wait = self.wait + op_code.cycles;
            self.pc = self.pc + op_code.length;
        } else {
            println!("CPU is waiting...");
        }

        self.wait = self.wait - 1;
    }
}

fn main() {
    let mut bus = MemoryBus { memory: [0x00; 0x10000] };

    let mut cpu = CPU {
        registers: Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0
        },
        pc: 0,
        sp: 0,
        wait: 0,
        bus: &mut bus,
    };

    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
    cpu.clock();
}
