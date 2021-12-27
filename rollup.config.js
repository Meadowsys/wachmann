import alias from "@rollup/plugin-alias";
import cjs from "@rollup/plugin-commonjs";
import json from "@rollup/plugin-json";
import node from "@rollup/plugin-node-resolve";
import replace from "@rollup/plugin-replace";
import fs from "fs";
import path from "path";
import license from "rollup-plugin-license";
import notify from "rollup-plugin-notify";
import { terser } from "rollup-plugin-terser";
import ts from "rollup-plugin-typescript2";

const production = process.env.NODE_ENV === "production";
const version = JSON.parse(fs.readFileSync(path.resolve("./package.json"), "utf8")).version;

/** @type {import("rollup").RollupOptions} */
const config = {
	watch: { clearScreen: false },
	input: path.resolve("./src/db-service/index.ts"),
	output: {
		file: path.resolve(`./target/${production ? "release" : "debug"}/db-service.mjs`),
		format: "es",
		compact: production,
		sourcemap: "inline",
		inlineDynamicImports: true
	},
	plugins: [
		// resolve node imports
		node(),

		// resolve json module
		json(),

		// allow importing of commonjs modules
		cjs(),

		// typescript support
		ts(),

		// create aliases for some dependencies
		alias({
			entries: []
		}),

		// some constants
		replace({
			preventAssignment: true,
			"process.env.NODE_ENV": JSON.stringify(production ? "production" : "development"),
			"__VERSION__": JSON.stringify(version)
		}),

		// replace these dotenv variables in production
		production && replace({
			preventAssignment: true,
			"process.env.DOTENV_CONFIG_ENCODING": JSON.stringify(null),
			"process.env.DOTENV_CONFIG_PATH": JSON.stringify(null),
			"process.env.DOTENV_CONFIG_DEBUG": JSON.stringify(null)
		}),

		// squeeze for production
		production && terser({
			ecma: 2020,
			compress: {
				passes: 3
			},
			mangle: {
				toplevel: true
			}
		}),

		// append license header in production
		production && license({
			sourcemap: true,
			banner: {
				content: `
					wachmann (v${version}): moderation bot for Galacon's Discord server
					Copyright (c) 2021 Autumn Meadow (autumnblazey), Pony Events Federation e.V.

					This program is free software: you can redistribute it and/or modify
					it under the terms of the GNU Affero General Public License as
					published by the Free Software Foundation, version 3 only.

					This program is distributed in the hope that it will be useful,
					but WITHOUT ANY WARRANTY; without even the implied warranty of
					MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
					GNU Affero General Public License for more details.

					You should have received a copy of the GNU Affero General Public License
					along with this program.  If not, see <https://www.gnu.org/licenses/>.
				`
					.trim()
					.replace("\r\n", "\n")
					.split("\n")
					.map(s => s.trimStart())
					.join("\n"),

				commentStyle: "ignored"
			}
		}),

		// send notifications in dev mode
		!production && notify({ success: true })
	]
};

export default config;
