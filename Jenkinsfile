pipeline {
    agent { label 'rust' }
    stages {
        stage('Prepare workspace') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
          steps {
            sh 'cargo clean'
            sh 'cross clean'
          }
        }
        stage('Unit tests') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
          steps {
            sh 'cargo test --release'
          }
        }
        stage('Build Linux x86-64') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
            steps {
              sh 'cargo build --release'
              sh 'upx target/release/cli'
          }
        }
        stage('Build Windows x86-64') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
          steps {
            sh 'cross build --release --target x86_64-pc-windows-gnu'
            sh 'upx --force target/x86_64-pc-windows-gnu/release/cli.exe'
          }
        }
        stage('Build Windows x86') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
          steps {
            sh 'cross build --release --target i686-pc-windows-gnu'
            sh 'upx --force target/i686-pc-windows-gnu/release/cli.exe'
          }
        }
        stage('Build ARMv7 (MiSTer native)') {
          environment {
            PATH="/home/jenkins/.cargo/bin:$PATH"
          } 
          steps {
            sh 'cross build --release --target armv7-unknown-linux-musleabihf'
            sh 'upx target/armv7-unknown-linux-musleabihf/release/cli'
          }
        }
        stage('Malware scan') {
          steps {
            sh 'clamscan -r -i /home/jenkins/workspace'
          }
        }
        stage('Package') {
          steps {
            sh 'mkdir upload'
            sh 'mkdir upload/linux_armv7'
            sh 'mkdir upload/linux_x86_64'
            sh 'mkdir upload/windows_x86_64'
            sh 'mkdir upload/windows_x86'
            sh 'cp target/armv7-unknown-linux-musleabihf/release/cli upload/linux_armv7/doscontainer'
            sh 'cp target/i686-pc-windows-gnu/release/cli.exe upload/windows_x86/doscontainer.exe'
            sh 'cp target/x86_64-pc-windows-gnu/release/cli.exe upload/windows_x86_64/doscontainer.exe'
            sh 'cp target/release/cli upload/linux_x86_64/doscontainer'
            sh 'cd upload ; zip -r ../doscontainer-${BUILD_NUMBER}.zip ./'
            sh 'cd upload ; gpg --detach-sign -a ../doscontainer-${BUILD_NUMBER}.zip'
          }
        }
        stage('Publish ZIP file') {
            steps {
                sshagent(credentials: ['ftp-uploader']) {
                    sh 'scp ./doscontainer-${BUILD_NUMBER}.zip uploader@10.20.0.17:/srv/ftp/doscontainer/builds/doscontainer-${BUILD_NUMBER}.zip'
                    sh 'scp ./doscontainer-${BUILD_NUMBER}.zip.asc uploader@10.20.0.17:/srv/ftp/doscontainer/builds/doscontainer-${BUILD_NUMBER}.zip.asc'
                }
            }
        }
    }
    post {
      // Clean after build
      always {
        cleanWs()
      }
    }
}
