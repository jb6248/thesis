\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef bass
        \time 4/4
        \absolute {
          <a, c e>2 <b, ees fis>2 <e, g b>2
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
