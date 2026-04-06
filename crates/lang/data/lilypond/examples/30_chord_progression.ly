\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <f' aes'>2 <c' e'>2 <e' g'>2 <d' fis'>2 <fis' a'>2 <d' b'>2 <d' b'>2 <c' e'>2
        }
        \bar "|."
      }
      \new Staff {
        \clef bass
        \time 4/4
        \absolute {
          d2 a2 c2 b2 d2 g2 g2 a2
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
