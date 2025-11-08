#include "cinder/app/App.h"
#include "cinder/app/RendererGl.h"
#include "cinder/gl/gl.h"

using namespace ci;
using namespace ci::app;
using namespace std;
const int WINDOW_WIDTH = 800;
const int WINDOW_HEIGHT = 800;
int t = 0;
float recursionLimitLength;

class SnowflakeFractalApp : public App {
  public:
	static void prepareSettings(Settings* settings);
	void draw() override;
    static void SnowflakeFractalApp::kochFractal(float xa, float ya, float xd, float yd);
};


void SnowflakeFractalApp::prepareSettings(Settings* settings)
{
	settings->setWindowSize(WINDOW_WIDTH, WINDOW_HEIGHT);
	settings->setTitle("Snowflake Fractal");
	settings->setFrameRate(2);
}

void SnowflakeFractalApp::draw()
{
	gl::clear( Color(0.831, 0.831, 0.831) );
    gl::color(0, 0, 0);
    gl::lineWidth(2);
	if (t == 0) { recursionLimitLength = 729; }
	else if (t == 1) { recursionLimitLength = 243; }
	else if (t == 2) { recursionLimitLength = 81; }
	else if (t == 3) { recursionLimitLength = 27; }
	else if (t == 4) { recursionLimitLength = 9; }
    SnowflakeFractalApp::kochFractal(100, 570, 700, 570);
    SnowflakeFractalApp::kochFractal(700, 570, 400, 50);
    SnowflakeFractalApp::kochFractal(400, 50, 100, 570);
	if (++t == 5) { t = 0; }
}

void SnowflakeFractalApp::kochFractal(float xa, float ya, float xd, float yd)
{
    float len = std::sqrt((xd - xa) * (xd - xa) + (yd - ya) * (yd - ya));
    if (len < recursionLimitLength)
    {
        gl::drawLine(vec2(xa, ya), vec2(xd, yd));
    }
    else
    {
        float xb = xa + (xd - xa) / 3.;
        float yb = ya + (yd - ya) / 3.;
        float xc = xa + (xd - xa) * 2. / 3.;
        float yc = ya + (yd - ya) * 2. / 3.;
        float xm = xa + (xd - xa) / 2.;
        float ym = ya + (yd - ya) / 2.;
        float sineTheta = (yd - ya) / len;
        float cosineTheta = (xd - xa) / len;
        float xk = xm - (len / 3.) * (sqrt(3) / 2.) * sineTheta;
        float yk = ym + (len / 3.) * (sqrt(3) / 2.) * cosineTheta;

        SnowflakeFractalApp::kochFractal(xa, ya, xb, yb);
        SnowflakeFractalApp::kochFractal(xb, yb, xk, yk);
        SnowflakeFractalApp::kochFractal(xk, yk, xc, yc);
        SnowflakeFractalApp::kochFractal(xc, yc, xd, yd);
    }
}

CINDER_APP( SnowflakeFractalApp, RendererGl, &SnowflakeFractalApp::prepareSettings)
