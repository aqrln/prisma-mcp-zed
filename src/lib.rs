use zed_extension_api as zed;

const PRISMA_PACKAGE: &str = "prisma";
const CLI_PATH: &str = "node_modules/prisma/build/index.js";

struct PrismaModelContextExtension;

impl zed::Extension for PrismaModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        let latest_version = "6.6.0-integration-mcp.5"; // replace with `zed::npm_package_latest_version` once released
        let installed_version = zed::npm_package_installed_version(PRISMA_PACKAGE)?;

        if installed_version.as_deref() != Some(latest_version) {
            zed::npm_install_package(PRISMA_PACKAGE, latest_version)?;
        }

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                std::env::current_dir()
                    .expect("current directory must exist and have valid permissions")
                    .join(CLI_PATH)
                    .to_str()
                    .expect("path must be valid utf-8")
                    .into(),
                "platform".into(),
                "mcp".into(),
                "--early-access".into(),
            ],
            env: std::env::vars().collect(),
        })
    }
}

zed::register_extension!(PrismaModelContextExtension);
