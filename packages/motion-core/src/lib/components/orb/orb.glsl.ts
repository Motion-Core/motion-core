import { simplex3D } from "./noise.glsl";

export const orbVertex = `
  varying vec3 vNormal, vObjPos, vWorldPos;
  void main(){
    vNormal = normalize(normalMatrix * normal);
    vObjPos = position;
    vWorldPos = (modelMatrix * vec4(position,1.0)).xyz;
    gl_Position = projectionMatrix * modelViewMatrix * vec4(position,1.0);
  }
`;

export const orbFragment = `
  uniform float uTime;
  uniform vec3 uLightPos;
  uniform float uAmplitude;
  uniform float uNoiseSpeed;
  uniform vec3 uFresnelColor;
  uniform float uFresnelStrength;

  uniform float uFlowDrift;
  uniform float uFlowOut;
  uniform float uFlowIn;
  uniform float uFlowPulse;
  uniform float uFlowSwirl;

  uniform vec3 uPalA0, uPalA1, uPalA2, uPalA3, uPalA4;
  uniform vec3 uPalB0, uPalB1, uPalB2, uPalB3, uPalB4;
  uniform float uPaletteBlend;

  varying vec3 vNormal, vObjPos, vWorldPos;

  ${simplex3D}

  vec3 paletteMix(vec3 c0, vec3 c1, vec3 c2, vec3 c3, vec3 c4, float n1, float n2, float n3, float n4){
    vec3 c = mix(c0, c1, smoothstep(.15, .65, n1));
    c = mix(c, c2, smoothstep(.2, .55, n2));
    c = mix(c, c3, smoothstep(.4, .75, n3) * .6);
    c = mix(c, c4, smoothstep(.1, .4, n4) * .4);
    return c;
  }

  void main(){
    vec3 N = normalize(vNormal);
    vec3 V = normalize(cameraPosition - vWorldPos);
    vec3 L = normalize(uLightPos - vWorldPos);

    float t = uTime * uNoiseSpeed;
    vec3 radial = normalize(vObjPos + vec3(0.001));
    vec3 tangent = normalize(cross(radial, vec3(0.0, 1.0, 0.001)));

    float audioActive = uFlowOut + uFlowIn + uFlowPulse;

    vec3 p = vObjPos * 1.2;

    // Idle: gentle meandering
    p += vec3(sin(t*0.3)*0.15, sin(t*0.2)*0.2, cos(t*0.25)*0.15) * uFlowDrift;

    // Dictation: strong vertical slide
    p += vec3(0.0, sin(t * 1.8) * 0.5 + sin(t * 2.9) * 0.15, 0.0) * uFlowOut;
    p += vec3(sin(t * 0.4) * 0.08, 0.0, cos(t * 0.3) * 0.08) * uFlowOut;

    // Listening: tangential swirl — colors visibly rotate around the sphere
    p += tangent * sin(t * 1.2) * 0.5 * uFlowIn;
    p += cross(tangent, radial) * cos(t * 0.8) * 0.3 * uFlowIn;
    p -= radial * sin(t * 1.6) * 0.15 * uFlowIn;

    // Talking: radial breathing — colors expand outward and contract back
    p += radial * sin(t * 2.0) * 0.55 * uFlowPulse;
    p += radial * sin(t * 3.3) * 0.2 * uFlowPulse;

    // Working: lissajous orbital churn
    p += vec3(sin(t*1.3)*0.35, cos(t*0.9)*0.3, sin(t*1.7)*0.35) * uFlowSwirl;
    p += tangent * sin(t*2.0) * 0.25 * uFlowSwirl;

    float warpAmt = 0.4 * (1.0 - audioActive * 0.5);
    vec3 warp1 = vec3(
      snoise(p + vec3(0.0, t * 0.12, 0.0)),
      snoise(p + vec3(5.2, t * 0.10, 1.3)),
      snoise(p + vec3(1.7, 3.4, t * 0.14))
    );
    p += warp1 * warpAmt;

    vec3 warp2 = vec3(
      snoise(p * 0.8 + vec3(t * 0.08, 0.0, 3.1)),
      snoise(p * 0.8 + vec3(8.3, t * 0.07, 0.0)),
      snoise(p * 0.8 + vec3(0.0, 2.8, t * 0.09))
    );
    vec3 fc = p + warp2 * 0.15;

    float ns = 1.0 + audioActive * 0.6;
    float n1 = snoise(fc * 0.7 * ns) * 0.5 + 0.5;
    float n2 = snoise(fc * 0.9 * ns + vec3(3.7, 1.2, 4.1)) * 0.5 + 0.5;
    float n3 = snoise(fc * 0.6 * ns + vec3(7.1, 8.3, 2.9)) * 0.5 + 0.5;
    float n4 = snoise(fc * 1.1 * ns + vec3(2.3, 5.1, 0.7)) * 0.5 + 0.5;

    vec3 colA = paletteMix(uPalA0, uPalA1, uPalA2, uPalA3, uPalA4, n1, n2, n3, n4);
    vec3 colB = paletteMix(uPalB0, uPalB1, uPalB2, uPalB3, uPalB4, n1, n2, n3, n4);
    vec3 col = mix(colA, colB, uPaletteBlend);

    float diff = max(dot(N, L), 0.0) * 0.5 + 0.5;
    vec3 H = normalize(L + V);
    float spec = pow(max(dot(N, H), 0.0), 48.0) * 0.4;

    vec3 fCol = uFresnelColor;
    float fStr = uFresnelStrength;
    col = mix(col, fCol, pow(1.0 - max(dot(N, V), 0.0), 2.5) * 0.18 * fStr);

    vec3 fin = col * diff + vec3(1.0) * spec + fCol * pow(1.0 - max(dot(N, V), 0.0), 3.0) * 0.35 * fStr;
    fin += fCol * pow(1.0 - max(dot(N, V), 0.0), 3.0) * 0.12 * fStr;

    gl_FragColor = vec4(fin, 1.0);
  }
`;
