# 離散フーリエ変換
## プログラムの動作

- `dft`: 離散フーリエ変換を行います．
- `dump_wav`: WAVE ファイルをダンプして，1列目に時間，2列目に振幅，3列目にDFTしたものをinv-DFTしたものを表示します．(動作確認用)
- `generate_wav`: WAVE ファイルを生成します．

## DFT

- `sample.wav` を離散フーリエ変換します．
- 結果は [-1.0,1.0] に正規化されます．
- WAVEファイルの量子化ビット数は16bitを想定しています．

## dump_wav

- `sample.wav` の振幅をダンプします．
- 2列目と3列目を比較することで逆離散フーリエ変換が正しく動作することを確認できます．不要な場合はソースを変更してください．

## generate_wav

- [-1.0,1.0] の信号をサンプリングレート44.1 kHz で WAVE ファイル `sample.wav` に出力します．
- 振幅の最大絶対値が1を超えないように信号の正規化を必ず行ってください．

## ライブラリ
このプログラムは WAVE ファイルのエンコード・デコードにライブラリ Hound を使用しています．
https://github.com/ruuda/hound

このライブラリは Apache License, Version 2.0 の下公開されています．
http://www.apache.org/licenses/LICENSE-2.0

## ライセンス
このプログラムは MIT Licenses の下公開されています．詳細は LICENSE ファイルをご覧ください．
