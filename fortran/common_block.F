c -*-Fortran-*-
      FUNCTION hepevt_dummy_function()
      include "hepevt.h"
      integer hepevt_dummy_function
      hepevt_dummy_function = NMXHEP
      NEVHEP = 43

      JMOHEP(1,1) = 11
      JMOHEP(2,1) = 21

      JMOHEP(1,3) = 13
      JMOHEP(2,3) = 23
      end