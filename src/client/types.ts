import { Commitment } from '@solana/web3.js';
export interface initializeStampInput {
  reference: string;
  authority: string;
  feePayer: string;
}

export interface WithdrawalInput {
  source: string;
  destination: string;
  mint: string;
  reference: string;
  amount: string;
  feeBps?: number;
  fixedFee?: string;
  memo?: string;
  commitment?: Commitment;
  computeUnitPrice?: number;
  computeBudget?: number;
}

export interface EscrowInput {
  escrowAddress: string;
  memo?: string;
  commitment?: Commitment;
  computeUnitPrice?: number;
  computeBudget?: number;
}

export interface ResultContext {
  transaction: string;
  slot: number;
}
export interface SettleAndTransferInput {
  walletAddress: string;
  transferTokenMintAddress: string;
  amountToSettle: string;
  amountToTransfer: string;
  escrowAddress: string;
  memo?: string;
  fee?: string;
  computeUnitPrice?: number;
  computeBudget?: number;
}

export interface CancelPaymentOutput {
  signature: string;
}
