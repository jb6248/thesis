\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <e' aes'' b''>2 <e' g'' b''>2 <fis' cis'' a''>2 <d' fis'' a''>2 <fis' cis'' a''>2 <f' c'' aes''>2 <g' d'' bes''>2 <c' e'' g''>2
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
