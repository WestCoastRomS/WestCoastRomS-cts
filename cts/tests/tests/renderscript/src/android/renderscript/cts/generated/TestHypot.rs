/*
 * Copyright (C) 2016 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// Don't edit this file!  It is auto-generated by frameworks/rs/api/generate.sh.

#pragma version(1)
#pragma rs java_package_name(android.renderscript.cts)

rs_allocation gAllocInB;

float __attribute__((kernel)) testHypotFloatFloatFloat(float inA, unsigned int x) {
    float inB = rsGetElementAt_float(gAllocInB, x);
    return hypot(inA, inB);
}

float2 __attribute__((kernel)) testHypotFloat2Float2Float2(float2 inA, unsigned int x) {
    float2 inB = rsGetElementAt_float2(gAllocInB, x);
    return hypot(inA, inB);
}

float3 __attribute__((kernel)) testHypotFloat3Float3Float3(float3 inA, unsigned int x) {
    float3 inB = rsGetElementAt_float3(gAllocInB, x);
    return hypot(inA, inB);
}

float4 __attribute__((kernel)) testHypotFloat4Float4Float4(float4 inA, unsigned int x) {
    float4 inB = rsGetElementAt_float4(gAllocInB, x);
    return hypot(inA, inB);
}

half __attribute__((kernel)) testHypotHalfHalfHalf(half inA, unsigned int x) {
    half inB = rsGetElementAt_half(gAllocInB, x);
    return hypot(inA, inB);
}

half2 __attribute__((kernel)) testHypotHalf2Half2Half2(half2 inA, unsigned int x) {
    half2 inB = rsGetElementAt_half2(gAllocInB, x);
    return hypot(inA, inB);
}

half3 __attribute__((kernel)) testHypotHalf3Half3Half3(half3 inA, unsigned int x) {
    half3 inB = rsGetElementAt_half3(gAllocInB, x);
    return hypot(inA, inB);
}

half4 __attribute__((kernel)) testHypotHalf4Half4Half4(half4 inA, unsigned int x) {
    half4 inB = rsGetElementAt_half4(gAllocInB, x);
    return hypot(inA, inB);
}
