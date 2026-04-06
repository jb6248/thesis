\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <c' ees'' g''>2 <cis' e'' aes''>2 <e' g'' bes''>2 <fis' cis'' a''>2 <aes' c'' ees''>2 <e' g'' b''>2 <ees' fis'' bes''>2 <d' f'' a''>2
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
