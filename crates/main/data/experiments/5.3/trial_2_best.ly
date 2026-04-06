\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <e' g'' b''>2 <aes' ees'' b''>2 <aes' ees'' b''>2 <e' g'' b''>2 <e' g'' b''>2 <aes' ees'' b''>2 <ees' g'' bes''>2 <c' e'' g''>2
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
