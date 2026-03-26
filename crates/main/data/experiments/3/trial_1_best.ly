\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <aes' b'' ees''>2 <c' e'' g''>2 r2 <aes' b'' ees''>2 r1 r2 <aes' b'' ees''>2 r1 r1 r1 r2 <aes'' b'' e'>2 r1 r1 r1 r1 r1 r1 r1 r2 <aes' b'' ees''>2 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 <aes'' b'' e'>2 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r1 r2 <c' ees'' g''>2
        }
        \bar "|."
      }
      \new Staff {
        \clef bass
        \time 4/4
        \absolute {
          r1
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
