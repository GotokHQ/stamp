import { Borsh } from '@metaplex-foundation/mpl-core';

type Args = {
  bump: number;
};

export class InitStampArgs extends Borsh.Data<Args> {
  static readonly SCHEMA = InitStampArgs.struct([
    ['instruction', 'u8'],
    ['bump', 'u8'],
  ]);

  instruction = 0;
  bump: number;
}
