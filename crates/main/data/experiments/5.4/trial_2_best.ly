\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <e' g'' b''>2 <d' fis'' a''>2 <d' fis'' a''>2 <b' d'' fis''>2 <e' g'' b''>2 <g' cis'' bes''>2 <d' fis'' a''>2 <c' e'' g''>2
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
