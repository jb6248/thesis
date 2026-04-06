\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <fis' cis'' a''>2 <b' d'' fis''>2 <aes' ees'' b''>2 <cis' f'' aes''>2 <fis' cis'' a''>2 <c' e'' g''>2 <ees' fis'' a''>2 <c' e'' g''>2
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
