subroutine FIB(A,N)
    Integer N
    Real*8 A(N)
!f2py intent(in) n
!f2py intent(out) a
!f2py depend(n) a
    DO I=1,N
        IF (I.EQ.1) THEN
            A(I) = 0.0D0
        ELSEIF (I.EQ.2) THEN
            A(I) = 1.0D0
        ELSE 
            A(I) = A(I-1) + A(I-2)
        ENDIF
      ENDDO
end