import pygame, sys
from pygame.locals import *

global recursionLimitLength

FPS = 2
WINDOWWIDTH=800
WINDOWHEIGHT=800
BLACK=(0,0,0)
GRAY=(212,212,212)

def main():
    global MAINCLOCK, SCREEN
    pygame.init()
    SCREEN=pygame.display.set_mode((WINDOWWIDTH, WINDOWHEIGHT))
    SCREEN.fill(GRAY)
    pygame.display.update()
    pygame.display.set_caption("Snowflake Fractal")
    MAINCLOCK=pygame.time.Clock()
    level=1
    snowflakeFractal(level)
    pygame.display.update()
    level=level+1
    running=True
    while(running):
        for event in pygame.event.get():
            if(event.type==pygame.QUIT):
                running=False
        if(running==True):        
            snowflakeFractal(level)
            MAINCLOCK.tick(FPS)    
            pygame.display.update()
            level=level+1
            if(level<1 or level>5):
                level=1
    pygame.quit()
    sys.exit()

def snowflakeFractal(level):
    global recursionLimitLength
    if(level==1):
        recursionLimitLength=729
    elif(level==2):
        recursionLimitLength=243
    elif(level==3):
        recursionLimitLength=81
    elif(level==4):
        recursionLimitLength=27
    elif(level==5):
        recursionLimitLength=9
    SCREEN.fill(GRAY)    
    fractalSegment(100,570,700,570)
    fractalSegment(700,570,400,50)
    fractalSegment(400,50,100,570)
    

def fractalSegment(xa,ya,xd,yd):
    leng=((xd-xa)**2+(yd-ya)**2)**0.5
    if(leng<recursionLimitLength):
        pygame.draw.line(SCREEN,BLACK,(xa,ya),(xd,yd))
    else:
        xb=xa+(xd-xa)/3.
        yb=ya+(yd-ya)/3.
        xc=xa+(xd-xa)*2./3.
        yc=ya+(yd-ya)*2./3.
        xm=xa+(xd-xa)/2.
        ym=ya+(yd-ya)/2.
        sineTheta=(yd-ya)/leng
        cosineTheta=(xd-xa)/leng
        xk=xm-(leng/3.)*((3**0.5)/2.)*sineTheta
        yk=ym+(leng/3.)*((3**0.5)/2.)*cosineTheta

        fractalSegment(xa,ya,xb,yb)
        fractalSegment(xb,yb,xk,yk)
        fractalSegment(xk,yk,xc,yc)
        fractalSegment(xc,yc,xd,yd)

if __name__ == '__main__':
    main()
