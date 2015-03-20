/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

enum CanvasWindingRule { "nonzero", "evenodd" };

// http://www.whatwg.org/html/#2dcontext
//[Constructor(optional unsigned long width, unsigned long height), Exposed=Window,Worker]
interface CanvasRenderingContext2D {

  // back-reference to the canvas
  readonly attribute HTMLCanvasElement canvas;

  // canvas dimensions
  //         attribute unsigned long width;
  //         attribute unsigned long height;

  // for contexts that aren't directly fixed to a specific canvas
  //void commit(); // push the image to the output bitmap

  // state
  //void save(); // push state on state stack
  //void restore(); // pop state stack and restore state

  // transformations (default transform is the identity matrix)
  //         attribute SVGMatrix currentTransform;
  void scale(/*unrestricted*/ double x, /*unrestricted*/ double y);
  //void rotate(unrestricted double angle);
  void translate(/*unrestricted*/ double x, /*unrestricted*/ double y);
  void transform(/*unrestricted*/ double a,
                 /*unrestricted*/ double b,
                 /*unrestricted*/ double c,
                 /*unrestricted*/ double d,
                 /*unrestricted*/ double e,
                 /*unrestricted*/ double f);
  void setTransform(/*unrestricted*/ double a,
                    /*unrestricted*/ double b,
                    /*unrestricted*/ double c,
                    /*unrestricted*/ double d,
                    /*unrestricted*/ double e,
                    /*unrestricted*/ double f);
  //void resetTransform();

  // compositing
  //         attribute unrestricted double globalAlpha; // (default 1.0)
  //         attribute DOMString globalCompositeOperation; // (default source-over)

  // image smoothing
  //         attribute boolean imageSmoothingEnabled; // (default true)

  // colours and styles (see also the CanvasDrawingStyles interface)
           attribute (DOMString or CanvasGradient or CanvasPattern) strokeStyle; // (default black)
           attribute (DOMString or CanvasGradient or CanvasPattern) fillStyle; // (default black)
  [Throws]
  CanvasGradient createLinearGradient(double x0, double y0, double x1, double y1);
  [Throws]
  CanvasGradient createRadialGradient(double x0, double y0, double r0, double x1, double y1, double r1);
  //CanvasPattern createPattern(CanvasImageSource image, [TreatNullAs=EmptyString] DOMString repetition);

  // shadows
  //         attribute unrestricted double shadowOffsetX; // (default 0)
  //         attribute unrestricted double shadowOffsetY; // (default 0)
  //         attribute unrestricted double shadowBlur; // (default 0)
  //         attribute DOMString shadowColor; // (default transparent black)

  // rects
  //void clearRect(unrestricted double x, unrestricted double y, unrestricted double w, unrestricted double h);
  //[LenientFloat]
  void clearRect(double x, double y, double w, double h);
  //void fillRect(unrestricted double x, unrestricted double y, unrestricted double w, unrestricted double h);
  //[LenientFloat]
  void fillRect(double x, double y, double w, double h);
  //void strokeRect(unrestricted double x, unrestricted double y, unrestricted double w, unrestricted double h);
  //[LenientFloat]
  void strokeRect(double x, double y, double w, double h);

  // path API (see also CanvasPathMethods)
  void beginPath();
  void fill(optional CanvasWindingRule fillRule = "nonzero");
  //void fill(Path2D path, optional CanvasWindingRule fillRule = "nonzero");
  //void stroke();
  //void stroke(Path2D path);
  //void drawSystemFocusRing(Element element);
  //void drawSystemFocusRing(Path2D path, Element element);
  //boolean drawCustomFocusRing(Element element);
  //boolean drawCustomFocusRing(Path2D path, Element element);
  //void scrollPathIntoView();
  //void scrollPathIntoView(Path2D path);
  //void clip(optional CanvasWindingRule fillRule = "nonzero");
  //void clip(Path2D path, optional CanvasWindingRule fillRule = "nonzero");
  //void resetClip();
  //boolean isPointInPath(unrestricted double x, unrestricted double y, optional CanvasWindingRule fillRule = "nonzero");
  //boolean isPointInPath(Path2D path, unrestricted double x, unrestricted double y, optional CanvasWindingRule fillRule = "nonzero");
  //boolean isPointInStroke(unrestricted double x, unrestricted double y);
  //boolean isPointInStroke(Path2D path, unrestricted double x, unrestricted double y);

  // text (see also the CanvasDrawingStyles interface)
  //void fillText(DOMString text, unrestricted double x, unrestricted double y, optional unrestricted double maxWidth);
  //void strokeText(DOMString text, unrestricted double x, unrestricted double y, optional unrestricted double maxWidth);
  //TextMetrics measureText(DOMString text);

  // drawing images
  //void drawImage(CanvasImageSource image, unrestricted double dx, unrestricted double dy);
  //void drawImage(CanvasImageSource image, unrestricted double dx, unrestricted double dy, unrestricted double dw, unrestricted double dh);
  //void drawImage(CanvasImageSource image, unrestricted double sx, unrestricted double sy, unrestricted double sw, unrestricted double sh, unrestricted double dx, unrestricted double dy, unrestricted double dw, unrestricted double dh);

  // hit regions
  //void addHitRegion(optional HitRegionOptions options);
  //void removeHitRegion(DOMString id);

  // pixel manipulation
  [Throws]
  ImageData createImageData(double sw, double sh);
  [Throws]
  ImageData createImageData(ImageData imagedata);
  [Throws]
  ImageData getImageData(double sx, double sy, double sw, double sh);
  void putImageData(ImageData imagedata, double dx, double dy);
  void putImageData(ImageData imagedata, double dx, double dy, double dirtyX, double dirtyY, double dirtyWidth, double dirtyHeight);
};

[NoInterfaceObject]
interface CanvasPathMethods {
  // shared path API methods
  void closePath();
  void moveTo(/*unrestricted*/ double x, /*unrestricted*/ double y);
  void lineTo(double x, double y);
  void quadraticCurveTo(double cpx, double cpy, double x, double y);

  void bezierCurveTo(/*unrestricted*/ double cp1x,
                     /*unrestricted*/ double cp1y,
                     /*unrestricted*/ double cp2x,
                     /*unrestricted*/ double cp2y,
                     /*unrestricted*/ double x,
                     /*unrestricted*/ double y);

  //void arcTo(double x1, double y1, double x2, double y2, double radius); 
// NOT IMPLEMENTED  [LenientFloat] void arcTo(double x1, double y1, double x2, double y2, double radiusX, double radiusY, double rotation);

  //void rect(double x, double y, double w, double h);

  void arc(double x, double y, double radius, double startAngle, double endAngle, optional boolean anticlockwise = false);
// NOT IMPLEMENTED  [LenientFloat] void ellipse(double x, double y, double radiusX, double radiusY, double rotation, double startAngle, double endAngle, boolean anticlockwise);
};


CanvasRenderingContext2D implements CanvasPathMethods;

