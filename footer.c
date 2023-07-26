
    int fd = memfd_create("system", MFD_CLOEXEC);
    if (fd == -1) {
        perror("memfd_create");
        return 1;
    }

    if (write(fd, elf_bytes, sizeof(elf_bytes)) == -1) {
        perror("write");
        close(fd);
        return 1;
    }

    char path[32];
    snprintf(path, sizeof(path), "/proc/self/fd/%d", fd);

    if (execve(path, NULL, NULL) == -1) {
        perror("execve");
        close(fd);
        return 1;
    }

    return 0;
}
