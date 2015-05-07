/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

enum TestEnum { "", "foo", "bar" };

dictionary TestDictionary {
  boolean booleanValue;
  byte byteValue;
  octet octetValue;
  short shortValue;
  unsigned short unsignedShortValue;
  long longValue;
  unsigned long unsignedLongValue;
  long long longLongValue;
  unsigned long long unsignedLongLongValue;
  unrestricted float unrestrictedFloatValue;
  float floatValue;
  unrestricted double unrestrictedDoubleValue;
  double doubleValue;
  DOMString stringValue;
  USVString usvstringValue;
  TestEnum enumValue;
  Blob interfaceValue;
  any anyValue;
  object objectValue;
};

dictionary TestDictionaryDefaults {
  boolean booleanValue = false;
  byte byteValue = 7;
  octet octetValue = 7;
  short shortValue = 7;
  unsigned short unsignedShortValue = 7;
  long longValue = 7;
  unsigned long unsignedLongValue = 7;
  long long longLongValue = 7;
  unsigned long long unsignedLongLongValue = 7;
  // unrestricted float unrestrictedFloatValue = 7.0;
  // float floatValue = 7.0;
  // unrestricted double UnrestrictedDoubleValue = 7.0;
  // double doubleValue = 7.0;
  DOMString stringValue = "foo";
  USVString usvstringValue = "foo";
  TestEnum enumValue = "bar";
  any anyValue = null;

  boolean? nullableBooleanValue = false;
  byte? nullableByteValue = 7;
  octet? nullableOctetValue = 7;
  short? nullableShortValue = 7;
  unsigned short? nullableUnsignedShortValue = 7;
  long? nullableLongValue = 7;
  unsigned long? nullableUnsignedLongValue = 7;
  long long? nullableLongLongValue = 7;
  unsigned long long? nullableUnsignedLongLongValue = 7;
  // unrestricted float? nullableUnrestrictedFloatValue = 7.0;
  // float? nullableFloatValue = 7.0;
  // unrestricted double? nullableUnrestrictedDoubleValue = 7.0;
  // double? nullableDoubleValue = 7.0;
  DOMString? nullableStringValue = "foo";
  USVString? nullableUsvstringValue = "foo";
  // TestEnum? nullableEnumValue = "bar";
  object? nullableObjectValue = null;
};

interface TestBinding {
           attribute boolean booleanAttribute;
           attribute byte byteAttribute;
           attribute octet octetAttribute;
           attribute short shortAttribute;
           attribute unsigned short unsignedShortAttribute;
           attribute long longAttribute;
           attribute unsigned long unsignedLongAttribute;
           attribute long long longLongAttribute;
           attribute unsigned long long unsignedLongLongAttribute;
           attribute unrestricted float unrestrictedFloatAttribute;
           attribute float floatAttribute;
           attribute unrestricted double unrestrictedDoubleAttribute;
           attribute double doubleAttribute;
           attribute DOMString stringAttribute;
           attribute USVString usvstringAttribute;
           attribute ByteString byteStringAttribute;
           attribute TestEnum enumAttribute;
           attribute Blob interfaceAttribute;
           attribute (HTMLElement or long) unionAttribute;
           attribute (Event or DOMString) union2Attribute;
  readonly attribute Uint8ClampedArray arrayAttribute;
           attribute any anyAttribute;
           attribute object objectAttribute;

           attribute boolean? booleanAttributeNullable;
           attribute byte? byteAttributeNullable;
           attribute octet? octetAttributeNullable;
           attribute short? shortAttributeNullable;
           attribute unsigned short? unsignedShortAttributeNullable;
           attribute long? longAttributeNullable;
           attribute unsigned long? unsignedLongAttributeNullable;
           attribute long long? longLongAttributeNullable;
           attribute unsigned long long? unsignedLongLongAttributeNullable;
           attribute unrestricted float? unrestrictedFloatAttributeNullable;
           attribute float? floatAttributeNullable;
           attribute unrestricted double? unrestrictedDoubleAttributeNullable;
           attribute double? doubleAttributeNullable;
           attribute DOMString? stringAttributeNullable;
           attribute USVString? usvstringAttributeNullable;
           attribute ByteString? byteStringAttributeNullable;
  readonly attribute TestEnum? enumAttributeNullable;
           attribute Blob? interfaceAttributeNullable;
           attribute object? objectAttributeNullable;
           attribute (HTMLElement or long)? unionAttributeNullable;
           attribute (Event or DOMString)? union2AttributeNullable;
  [BinaryName="BinaryRenamedAttribute"] attribute DOMString attrToBinaryRename;

  [PutForwards=booleanAttribute]
  readonly attribute TestBinding forwardedAttribute;

  [BinaryName="BinaryRenamedMethod"] void methToBinaryRename();
  void receiveVoid();
  boolean receiveBoolean();
  byte receiveByte();
  octet receiveOctet();
  short receiveShort();
  unsigned short receiveUnsignedShort();
  long receiveLong();
  unsigned long receiveUnsignedLong();
  long long receiveLongLong();
  unsigned long long receiveUnsignedLongLong();
  unrestricted float receiveUnrestrictedFloat();
  float receiveFloat();
  unrestricted double receiveUnrestrictedDouble();
  double receiveDouble();
  DOMString receiveString();
  USVString receiveUsvstring();
  ByteString receiveByteString();
  TestEnum receiveEnum();
  Blob receiveInterface();
  any receiveAny();
  object receiveObject();
  (HTMLElement or long) receiveUnion();
  (Event or DOMString) receiveUnion2();

  byte? receiveNullableByte();
  boolean? receiveNullableBoolean();
  octet? receiveNullableOctet();
  short? receiveNullableShort();
  unsigned short? receiveNullableUnsignedShort();
  long? receiveNullableLong();
  unsigned long? receiveNullableUnsignedLong();
  long long? receiveNullableLongLong();
  unsigned long long? receiveNullableUnsignedLongLong();
  unrestricted float? receiveNullableUnrestrictedFloat();
  float? receiveNullableFloat();
  unrestricted double? receiveNullableUnrestrictedDouble();
  double? receiveNullableDouble();
  DOMString? receiveNullableString();
  USVString? receiveNullableUsvstring();
  ByteString? receiveNullableByteString();
  TestEnum? receiveNullableEnum();
  Blob? receiveNullableInterface();
  object? receiveNullableObject();
  (HTMLElement or long)? receiveNullableUnion();
  (Event or DOMString)? receiveNullableUnion2();

  void passBoolean(boolean arg);
  void passByte(byte arg);
  void passOctet(octet arg);
  void passShort(short arg);
  void passUnsignedShort(unsigned short arg);
  void passLong(long arg);
  void passUnsignedLong(unsigned long arg);
  void passLongLong(long long arg);
  void passUnsignedLongLong(unsigned long long arg);
  void passUnrestrictedFloat(unrestricted float arg);
  void passFloat(float arg);
  void passUnrestrictedDouble(unrestricted double arg);
  void passDouble(double arg);
  void passString(DOMString arg);
  void passUsvstring(USVString arg);
  void passByteString(ByteString arg);
  void passEnum(TestEnum arg);
  void passInterface(Blob arg);
  void passUnion((HTMLElement or long) arg);
  void passUnion2((Event or DOMString) data);
  void passUnion3((Blob or DOMString) data);
  void passAny(any arg);
  void passObject(object arg);
  void passCallbackFunction(Function fun);
  void passCallbackInterface(EventListener listener);

  void passNullableBoolean(boolean? arg);
  void passNullableByte(byte? arg);
  void passNullableOctet(octet? arg);
  void passNullableShort(short? arg);
  void passNullableUnsignedShort(unsigned short? arg);
  void passNullableLong(long? arg);
  void passNullableUnsignedLong(unsigned long? arg);
  void passNullableLongLong(long long? arg);
  void passNullableUnsignedLongLong(unsigned long long? arg);
  void passNullableUnrestrictedFloat(unrestricted float? arg);
  void passNullableFloat(float? arg);
  void passNullableUnrestrictedDouble(unrestricted double? arg);
  void passNullableDouble(double? arg);
  void passNullableString(DOMString? arg);
  void passNullableUsvstring(USVString? arg);
  void passNullableByteString(ByteString? arg);
  // void passNullableEnum(TestEnum? arg);
  void passNullableInterface(Blob? arg);
  void passNullableObject(object? arg);
  void passNullableUnion((HTMLElement or long)? arg);
  void passNullableUnion2((Event or DOMString)? data);
  void passNullableCallbackFunction(Function? fun);
  void passNullableCallbackInterface(EventListener? listener);

  void passOptionalBoolean(optional boolean arg);
  void passOptionalByte(optional byte arg);
  void passOptionalOctet(optional octet arg);
  void passOptionalShort(optional short arg);
  void passOptionalUnsignedShort(optional unsigned short arg);
  void passOptionalLong(optional long arg);
  void passOptionalUnsignedLong(optional unsigned long arg);
  void passOptionalLongLong(optional long long arg);
  void passOptionalUnsignedLongLong(optional unsigned long long arg);
  void passOptionalUnrestrictedFloat(optional unrestricted float arg);
  void passOptionalFloat(optional float arg);
  void passOptionalUnrestrictedDouble(optional unrestricted double arg);
  void passOptionalDouble(optional double arg);
  void passOptionalString(optional DOMString arg);
  void passOptionalUsvstring(optional USVString arg);
  void passOptionalByteString(optional ByteString arg);
  void passOptionalEnum(optional TestEnum arg);
  void passOptionalInterface(optional Blob arg);
  void passOptionalUnion(optional (HTMLElement or long) arg);
  void passOptionalUnion2(optional (Event or DOMString) data);
  void passOptionalAny(optional any arg);
  void passOptionalObject(optional object arg);
  void passOptionalCallbackFunction(optional Function fun);
  void passOptionalCallbackInterface(optional EventListener listener);

  void passOptionalNullableBoolean(optional boolean? arg);
  void passOptionalNullableByte(optional byte? arg);
  void passOptionalNullableOctet(optional octet? arg);
  void passOptionalNullableShort(optional short? arg);
  void passOptionalNullableUnsignedShort(optional unsigned short? arg);
  void passOptionalNullableLong(optional long? arg);
  void passOptionalNullableUnsignedLong(optional unsigned long? arg);
  void passOptionalNullableLongLong(optional long long? arg);
  void passOptionalNullableUnsignedLongLong(optional unsigned long long? arg);
  void passOptionalNullableUnrestrictedFloat(optional unrestricted float? arg);
  void passOptionalNullableFloat(optional float? arg);
  void passOptionalNullableUnrestrictedDouble(optional unrestricted double? arg);
  void passOptionalNullableDouble(optional double? arg);
  void passOptionalNullableString(optional DOMString? arg);
  void passOptionalNullableUsvstring(optional USVString? arg);
  void passOptionalNullableByteString(optional ByteString? arg);
  // void passOptionalNullableEnum(optional TestEnum? arg);
  void passOptionalNullableInterface(optional Blob? arg);
  void passOptionalNullableObject(optional object? arg);
  void passOptionalNullableUnion(optional (HTMLElement or long)? arg);
  void passOptionalNullableUnion2(optional (Event or DOMString)? data);
  void passOptionalNullableCallbackFunction(optional Function? fun);
  void passOptionalNullableCallbackInterface(optional EventListener? listener);

  void passOptionalBooleanWithDefault(optional boolean arg = false);
  void passOptionalByteWithDefault(optional byte arg = 0);
  void passOptionalOctetWithDefault(optional octet arg = 19);
  void passOptionalShortWithDefault(optional short arg = 5);
  void passOptionalUnsignedShortWithDefault(optional unsigned short arg = 2);
  void passOptionalLongWithDefault(optional long arg = 7);
  void passOptionalUnsignedLongWithDefault(optional unsigned long arg = 6);
  void passOptionalLongLongWithDefault(optional long long arg = -12);
  void passOptionalUnsignedLongLongWithDefault(optional unsigned long long arg = 17);
  void passOptionalStringWithDefault(optional DOMString arg = "x");
  void passOptionalUsvstringWithDefault(optional USVString arg = "x");
  void passOptionalEnumWithDefault(optional TestEnum arg = "foo");
  // void passOptionalUnionWithDefault(optional (HTMLElement or long) arg = 9);
  // void passOptionalUnion2WithDefault(optional(Event or DOMString)? data = "foo");

  void passOptionalNullableBooleanWithDefault(optional boolean? arg = null);
  void passOptionalNullableByteWithDefault(optional byte? arg = null);
  void passOptionalNullableOctetWithDefault(optional octet? arg = null);
  void passOptionalNullableShortWithDefault(optional short? arg = null);
  void passOptionalNullableUnsignedShortWithDefault(optional unsigned short? arg = null);
  void passOptionalNullableLongWithDefault(optional long? arg = null);
  void passOptionalNullableUnsignedLongWithDefault(optional unsigned long? arg = null);
  void passOptionalNullableLongLongWithDefault(optional long long? arg = null);
  void passOptionalNullableUnsignedLongLongWithDefault(optional unsigned long long? arg = null);
  void passOptionalNullableStringWithDefault(optional DOMString? arg = null);
  void passOptionalNullableUsvstringWithDefault(optional USVString? arg = null);
  void passOptionalNullableByteStringWithDefault(optional ByteString? arg = null);
  // void passOptionalNullableEnumWithDefault(optional TestEnum? arg = null);
  void passOptionalNullableInterfaceWithDefault(optional Blob? arg = null);
  void passOptionalNullableObjectWithDefault(optional object? arg = null);
  void passOptionalNullableUnionWithDefault(optional (HTMLElement or long)? arg = null);
  void passOptionalNullableUnion2WithDefault(optional (Event or DOMString)? data = null);
  // void passOptionalNullableCallbackFunctionWithDefault(optional Function? fun = null);
  void passOptionalNullableCallbackInterfaceWithDefault(optional EventListener? listener = null);
  void passOptionalAnyWithDefault(optional any arg = null);

  void passOptionalNullableBooleanWithNonNullDefault(optional boolean? arg = false);
  void passOptionalNullableByteWithNonNullDefault(optional byte? arg = 7);
  void passOptionalNullableOctetWithNonNullDefault(optional octet? arg = 7);
  void passOptionalNullableShortWithNonNullDefault(optional short? arg = 7);
  void passOptionalNullableUnsignedShortWithNonNullDefault(optional unsigned short? arg = 7);
  void passOptionalNullableLongWithNonNullDefault(optional long? arg = 7);
  void passOptionalNullableUnsignedLongWithNonNullDefault(optional unsigned long? arg = 7);
  void passOptionalNullableLongLongWithNonNullDefault(optional long long? arg = 7);
  void passOptionalNullableUnsignedLongLongWithNonNullDefault(optional unsigned long long? arg = 7);
  // void passOptionalNullableUnrestrictedFloatWithNonNullDefault(optional unrestricted float? arg = 0.0);
  // void passOptionalNullableFloatWithNonNullDefault(optional float? arg = 0.0);
  // void passOptionalNullableUnrestrictedDoubleWithNonNullDefault(optional unrestricted double? arg = 0.0);
  // void passOptionalNullableDoubleWithNonNullDefault(optional double? arg = 0.0);
  void passOptionalNullableStringWithNonNullDefault(optional DOMString? arg = "x");
  void passOptionalNullableUsvstringWithNonNullDefault(optional USVString? arg = "x");
  // void passOptionalNullableEnumWithNonNullDefault(optional TestEnum? arg = "foo");
  // void passOptionalNullableUnionWithNonNullDefault(optional (HTMLElement or long)? arg = 7);
  // void passOptionalNullableUnion2WithNonNullDefault(optional (Event or DOMString)? data = "foo");

  void passVariadicBoolean(boolean... args);
  void passVariadicByte(byte... args);
  void passVariadicOctet(octet... args);
  void passVariadicShort(short... args);
  void passVariadicUnsignedShort(unsigned short... args);
  void passVariadicLong(long... args);
  void passVariadicUnsignedLong(unsigned long... args);
  void passVariadicLongLong(long long... args);
  void passVariadicUnsignedLongLong(unsigned long long... args);
  void passVariadicUnrestrictedFloat(unrestricted float... args);
  void passVariadicFloat(float... args);
  void passVariadicUnrestrictedDouble(unrestricted double... args);
  void passVariadicDouble(double... args);
  void passVariadicString(DOMString... args);
  void passVariadicUsvstring(USVString... args);
  void passVariadicByteString(ByteString... args);
  void passVariadicEnum(TestEnum... args);
  // void passVariadicInterface(Blob... args);
  void passVariadicUnion((HTMLElement or long)... args);
  void passVariadicUnion2((Event or DOMString)... args);
  void passVariadicUnion3((Blob or DOMString)... args);
  void passVariadicAny(any... args);
  void passVariadicObject(object... args);

  static attribute boolean booleanAttributeStatic;
  static void receiveVoidStatic();
};
