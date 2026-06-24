package io.github.noahhhx.site.controller;

import io.github.noahhhx.site.service.MarkdownService;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.nio.file.Files;
import lombok.RequiredArgsConstructor;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.util.ResourceUtils;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestMapping;

@Controller
@RequestMapping("/cv")
@RequiredArgsConstructor
public class CvController {
    
    private final MarkdownService markdownService;

    @GetMapping
    public String cv(Model model) throws IOException {
        model.addAttribute(
              "mdHtmlContent",
              markdownService.markdownToHtml(
                    Files.readString(
                          ResourceUtils.getFile("classpath:cv.md").toPath())));
        return "cv";
    }
}
