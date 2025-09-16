pub enum EnumWithMustUseMethods {
    Bar,
}

impl EnumWithMustUseMethods {
    // These methods did not have the #[must_use] attribute in the old version.
    // Addition of the attribute should NOT be reported.
    #[must_use]
    pub fn AssociatedFnToMustUse() {}

    #[must_use = "Foo"]
    pub fn AssociatedFnToMustUseMessage() {}

    #[must_use]
    pub fn MethodToMustUseMethod(&self) {}

    #[must_use = "Foo"]
    pub fn MethodToMustUseMessageMethod(&self) {}

    // This method had the #[must_use] attribute in the old version. Changes of
    // the attribute should not be reported.

    #[must_use = "Foo"]
    pub fn MustUseMethodToMustUseMessageMethod(&self) {}

    // These methods had the #[must_use] attribute in the old version.
    // They also included the user-defined warning message. Changes of
    // the attribute should not be reported.

    #[must_use]
    pub fn MustUseMessageMethodToMustUseMethod(&self) {}

    #[must_use = "Baz"]
    pub fn MustUseMessageMethodToMustUseMessageMethod(&self) {}


    // These methods had the #[must_use] attribute in the old version.
    // Deletion of the attribute SHOULD be reported.
    pub fn MustUseMethodToMethod(&self) {}

    pub fn MustUseMessageMethodToMethod(&self) {}


}

// This public enum's inherent method had the #[must_use] attribute
// in the old version. Because the method is private, removing the attribute
// should NOT be reported.

pub enum EnumWithPrivateMustUseMethods {
    Bar,
}

impl EnumWithPrivateMustUseMethods {
    fn PrivateMustUseMethodToPrivateMethod(&self) {}
}

// This enum is private and removing #[must_use] from its inherent method
// should NOT be reported.

enum PrivateEnumWithMustUseMethods {
    Bar,
}

impl PrivateEnumWithMustUseMethods {
    fn PrivateEnumMustUseMethodToPrivateEnumMethod(&self) {}
}
