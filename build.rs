fn main() {
    winres::WindowsResource::new()
        .set_manifest(r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
            <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                <security>
                    <requestedPrivileges>
                        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                    </requestedPrivileges>
                </security>
            </trustInfo>
            </assembly>
        "#)
        .set_icon("assets/favicon.ico")
        .compile().unwrap()
}