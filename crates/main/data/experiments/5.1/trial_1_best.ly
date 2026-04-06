\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <aes' c'' ees''>2 <ees' g'' bes''>2 <aes' c'' ees''>2 <c' e'' g''>2 <aes' c'' ees''>2 <aes' c'' ees''>2 <aes' c'' ees''>2 <c' e'' g''>2
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
