\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <g' d'' b''>2 <f' c'' a''>2 <a' c'' e''>2 <f' c'' a''>2 <a' c'' e''>2 <e' aes'' b''>2 <a' c'' e''>2 <f' c'' a''>2
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
