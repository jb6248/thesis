\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <aes' ees'' b''>2 <g' d'' b''>2 <aes' ees'' b''>2 <e' g'' b''>2 <f' c'' a''>2 <cis' e'' aes''>2 <aes' ees'' b''>2 <g' d'' b''>2
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
