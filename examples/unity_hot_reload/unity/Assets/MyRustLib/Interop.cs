// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using My.Company;
#pragma warning restore 0105

namespace My.Company
{
    public static partial class InteropClass
    {
        public const string NativeLib = "unity_hot_reload.304303941";

        static InteropClass()
        {
        }


        [DllImport(NativeLib, CallingConvention = CallingConvention.Cdecl, EntryPoint = "do_math")]
        public static extern uint do_math(uint x);

    }



}
