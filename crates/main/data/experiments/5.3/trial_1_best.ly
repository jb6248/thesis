\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <aes' ees'' b''>2 <bes' d'' f''>2 <e' g'' b''>2 <f' c'' aes''>2 <a' c'' ees''>2 <c' e'' g''>2 <fis' cis'' a''>2 <f' c'' a''>2
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
