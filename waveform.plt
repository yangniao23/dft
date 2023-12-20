set datafile separator ","
version = 3
name = "waveform".version

set terminal pdf enhanced
set output name.".pdf"

set style fill solid
set autoscale
set size square
set key
set key outside right
set key width 0
set grid
set xr [0:0.002]
#set yr [-100:10]
set mxtics 10
set mytics
set xlabel "t [s]"
set ylabel "Amplitude"
#set logscale x

plot name.".dat" using 1:2 w l title "振幅"
