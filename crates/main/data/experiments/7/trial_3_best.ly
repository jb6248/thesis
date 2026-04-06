\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <c' e'' g''>2 <c' e'' g''>2 <c' e'' g''>2 <g' d'' b''>2 <c' e'' g''>2 <g' d'' b''>2 <fis' cis'' a''>2 <c' e'' g''>2
        }
        \bar "|."
      }
    >>
  >>
  \layout { }
  \midi {
    \tempo 4 = 120
  }
}
