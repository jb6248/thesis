\version "2.24.0"

\score {
  <<
    \new PianoStaff <<
      \new Staff {
        \clef treble
        \time 4/4
        \absolute {
          <e' g'' bes''>2 <a' c'' e''>2 <a' c'' e''>2 <a' c'' e''>2 <a' c'' e''>2 <a' c'' e''>2 <a' c'' e''>2 <e' g'' b''>2
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
