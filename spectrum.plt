set datafile separator ","
version = ""
name = "spectrum".version

set terminal pdf enhanced
set output name.".pdf"

set style fill solid
set autoscale
set size square
set key
set key outside right
set key width 0
set grid
set xr [0:5000]
#set yr [0:1.1]
set mxtics 10
set mytics
set xlabel "f [kHz]"
set ylabel "Amplitude Spectrum"
#set logscale x

# DFT で 1/6, 1/3, 1/2 -> 1/3, 2/3, 1 されているから3倍して出力
plot name.".dat" using 1:($2*3) w boxes title "スペクトル"
