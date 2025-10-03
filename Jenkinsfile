// Jenkinsfile for the furrk-cms-server (Single Server Workflow)

pipeline {
    agent any

    // --- Environment Variables ---
    // Notice the SSH variables have been removed.
    environment {
        DOCKERHUB_CREDENTIALS = 'DOCKERHUB_CREDENTIALS'
        DOCKER_USERNAME = 'ZuriTheChee'
        IMAGE_NAME = 'furrk-cms-server'
    }

    stages {
        // --- Stage 1: Build and Publish Development Image ---
        stage('Build and Publish Dev Image') {
            when {
                branch 'develop'
            }
            steps {
                script {
                    echo "Building for 'develop' branch..."
                    withCredentials([usernamePassword(credentialsId: DOCKERHUB_CREDENTIALS, usernameVariable: 'DOCKER_USER', passwordVariable: 'DOCKER_PASS')]) {
                        sh "docker login -u ${DOCKER_USER} -p ${DOCKER_PASS}"
                    }
                    def fullImageName = "${DOCKER_USERNAME}/${IMAGE_NAME}:latest"
                    sh "docker build -t ${fullImageName} ."
                    sh "docker push ${fullImageName}"
                    echo "Pushed Development Image: ${fullImageName}"
                }
            }
        }

        // --- Stage 2: Deploy to Development Environment (LOCAL) ---
        stage('Deploy to Development') {
            when {
                branch 'develop'
            }
            steps {
                echo "Deploying to local Docker instance..."
                sh """
                    cd /home/yagdrassyl/rust-services && \\
                    docker-compose -p rust_dev -f docker-compose.dev.yml pull web-server && \\
                    docker-compose -p rust_dev -f docker-compose.dev.yml up -d --no-deps web-server
                """
            }
        }
    }

    post {
        always {
            sh "docker logout"
        }
    }
}

