import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Seroset",
  description: "TODO",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en-US">
      <body>{children}</body>
    </html>
  );
}
