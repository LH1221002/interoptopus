// Automatically generated by Interoptopus.

#pragma warning disable 0105
using System;
using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using My.Company;
using My.Company.Common;
#pragma warning restore 0105

namespace My.Company.Common
{

    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Vec
    {
        public double x;
        public double z;
    }

    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate byte InteropDelegate_fn_u8_rval_u8(byte x0);

    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceBool
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceBool : IEnumerable<Bool>
    {
        public SliceBool(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceBool(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public Bool this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(Bool));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<Bool>(ptr);
            }
        }
        public Bool[] Copied
        {
            get
            {
                var rval = new Bool[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Bool> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceI32
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceI32 : IEnumerable<int>
    {
        public SliceI32(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceI32(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public int this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(int));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<int>(ptr);
            }
        }
        public int[] Copied
        {
            get
            {
                var rval = new int[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<int> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceU32
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceU32 : IEnumerable<uint>
    {
        public SliceU32(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceU32(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(uint));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<uint>(ptr);
            }
        }
        public uint[] Copied
        {
            get
            {
                var rval = new uint[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<uint> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceU8
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceU8 : IEnumerable<byte>
    {
        public SliceU8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceU8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public byte this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(byte));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<byte>(ptr);
            }
        }
        public byte[] Copied
        {
            get
            {
                var rval = new byte[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<byte> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may not be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceVec
    {
        ///Pointer to start of immutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceVec : IEnumerable<Vec>
    {
        public SliceVec(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceVec(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(Vec));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<Vec>(ptr);
            }
        }
        public Vec[] Copied
        {
            get
            {
                var rval = new Vec[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Vec> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutConstPtrI8
    {
        ///Pointer to start of mutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutConstPtrI8 : IEnumerable<IntPtr>
    {
        public SliceMutConstPtrI8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutConstPtrI8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public IntPtr this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(IntPtr));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<IntPtr>(ptr);
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(IntPtr));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                Marshal.StructureToPtr<IntPtr>(value, ptr, false);
            }
        }
        public IntPtr[] Copied
        {
            get
            {
                var rval = new IntPtr[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<IntPtr> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutU32
    {
        ///Pointer to start of mutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutU32 : IEnumerable<uint>
    {
        public SliceMutU32(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutU32(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public uint this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(uint));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<uint>(ptr);
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(uint));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                Marshal.StructureToPtr<uint>(value, ptr, false);
            }
        }
        public uint[] Copied
        {
            get
            {
                var rval = new uint[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<uint> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutU8
    {
        ///Pointer to start of mutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutU8 : IEnumerable<byte>
    {
        public SliceMutU8(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutU8(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public byte this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(byte));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<byte>(ptr);
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(byte));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                Marshal.StructureToPtr<byte>(value, ptr, false);
            }
        }
        public byte[] Copied
        {
            get
            {
                var rval = new byte[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<byte> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///A pointer to an array of data someone else owns which may be modified.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct SliceMutVec
    {
        ///Pointer to start of mutable data.
        IntPtr data;
        ///Number of elements.
        ulong len;
    }

    public partial struct SliceMutVec : IEnumerable<Vec>
    {
        public SliceMutVec(GCHandle handle, ulong count)
        {
            this.data = handle.AddrOfPinnedObject();
            this.len = count;
        }
        public SliceMutVec(IntPtr handle, ulong count)
        {
            this.data = handle;
            this.len = count;
        }
        public Vec this[int i]
        {
            get
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(Vec));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                return Marshal.PtrToStructure<Vec>(ptr);
            }
            set
            {
                if (i >= Count) throw new IndexOutOfRangeException();
                var size = Marshal.SizeOf(typeof(Vec));
                var ptr = new IntPtr(data.ToInt64() + i * size);
                Marshal.StructureToPtr<Vec>(value, ptr, false);
            }
        }
        public Vec[] Copied
        {
            get
            {
                var rval = new Vec[len];
                for (var i = 0; i < (int) len; i++) {
                    rval[i] = this[i];
                }
                return rval;
            }
        }
        public int Count => (int) len;
        public IEnumerator<Vec> GetEnumerator()
        {
            for (var i = 0; i < (int)len; ++i)
            {
                yield return this[i];
            }
        }
        IEnumerator IEnumerable.GetEnumerator()
        {
            return this.GetEnumerator();
        }
    }


    ///Option type containing boolean flag and maybe valid data.
    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct OptionVec
    {
        ///Element that is maybe valid.
        Vec t;
        ///Byte where `1` means element `t` is valid.
        byte is_some;
    }

    public partial struct OptionVec
    {
        public static OptionVec FromNullable(Vec? nullable)
        {
            var result = new OptionVec();
            if (nullable.HasValue)
            {
                result.is_some = 1;
                result.t = nullable.Value;
            }

            return result;
        }

        public Vec? ToNullable()
        {
            return this.is_some == 1 ? this.t : (Vec?)null;
        }
    }


    [Serializable]
    [StructLayout(LayoutKind.Sequential)]
    public partial struct Bool
    {
        byte value;
    }

    public partial struct Bool
    {
        public static readonly Bool True = new Bool { value =  1 };
        public static readonly Bool False = new Bool { value =  0 };
        public Bool(bool b)
        {
            value = (byte) (b ? 1 : 0);
        }
        public bool Is => value == 1;
    }


    [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
    public delegate uint MyCallbackNamespaced(uint value);



    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error): base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }

}
