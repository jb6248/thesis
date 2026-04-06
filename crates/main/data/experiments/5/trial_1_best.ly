\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <b' d'' fis''>2 <c' ees'' g''>2 <d' fis'' a''>2 <cis' e'' g''>2 <b' d'' fis''>2 <c' ees'' g''>2 <b' d'' fis''>2 <c' e'' g''>2
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
