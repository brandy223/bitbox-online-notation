/** @type {import('next').NextConfig} */
const nextConfig = {
    // server: {
    //     https: {
    //         key: fs.readFileSync(path.join(__dirname, 'key.pem')),
    //         cert: fs.readFileSync(path.join(__dirname, 'cert.pem')),
    //     }
    // }
    images: {
        remotePatterns: [
            {
                protocol: 'https',
                hostname: 'image.freepik.com',
                port: '',
                pathname: '/**',
            },
        ]
    }
};

export default nextConfig;
